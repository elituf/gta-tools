use crate::{
    gui::settings::BlockMethod,
    util::{
        consts::game::{EXE_ENHANCED, EXE_LEGACY},
        system_info::SystemInfo,
    },
};
use std::{
    error::Error,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use strum::{Display, EnumIter};
use windows::{
    Win32::{
        NetworkManagement::WindowsFirewall::{
            INetFwPolicy2, INetFwRule, NET_FW_ACTION_BLOCK, NET_FW_IP_PROTOCOL_ANY,
            NET_FW_RULE_DIR_OUT, NetFwPolicy2, NetFwRule,
        },
        System::Com::{
            CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
            CoUninitialize,
        },
    },
    core::BSTR,
};

const FILTER_NAME_EXE: &str = "[GTA Tools] Block outbound traffic for all of GTA V";
const FILTER_NAME_SAVE_SERVER: &str = "[GTA Tools] Block outbound traffic to Rockstar save server";

const INTERVAL: Duration = Duration::from_secs(3);

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, EnumIter)]
pub enum BlockedStatus {
    Blocked,
    Failed,
    Unblocked,
}

impl From<bool> for BlockedStatus {
    fn from(value: bool) -> Self {
        if value {
            Self::Blocked
        } else {
            Self::Unblocked
        }
    }
}

#[derive(Debug)]
pub struct GameNetworking {
    com_initialized: bool,
    pub blocked_status: BlockedStatus,
    timer: Instant,
    counting: bool,
}

impl Default for GameNetworking {
    fn default() -> Self {
        Self {
            blocked_status: if Self::is_save_server_blocked().unwrap() {
                Self::is_save_server_blocked().unwrap().into()
            } else {
                Self::is_exe_blocked().unwrap().into()
            },
            com_initialized: unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.is_ok(),
            timer: Instant::now(),
            counting: false,
        }
    }
}

impl Drop for GameNetworking {
    fn drop(&mut self) {
        if self.com_initialized {
            unsafe { CoUninitialize() };
        }
    }
}

enum Mode {
    EntireGame(PathBuf),
    SaveServer(String),
}

impl GameNetworking {
    fn block_generic(&self, mode: Mode) -> Result<(), Box<dyn Error>> {
        let policy: INetFwPolicy2 =
            unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }?;
        let rules = unsafe { policy.Rules() }?;
        let filter_name = match mode {
            Mode::EntireGame(_) => FILTER_NAME_EXE,
            Mode::SaveServer(_) => FILTER_NAME_SAVE_SERVER,
        };
        unsafe { rules.Remove(&BSTR::from(filter_name)) }?;
        let rule: INetFwRule = unsafe { CoCreateInstance(&NetFwRule, None, CLSCTX_INPROC_SERVER) }?;
        unsafe { rule.SetName(&BSTR::from(filter_name)) }?;
        match mode {
            Mode::EntireGame(exe_path) => {
                let exe_path = BSTR::from(exe_path.to_string_lossy().to_string());
                unsafe { rule.SetApplicationName(&exe_path) }?;
            }
            Mode::SaveServer(save_server_ip) => {
                unsafe { rule.SetRemoteAddresses(&BSTR::from(save_server_ip)) }?;
            }
        }
        unsafe { rule.SetDirection(NET_FW_RULE_DIR_OUT) }?;
        unsafe { rule.SetEnabled(true.into()) }?;
        unsafe { rule.SetAction(NET_FW_ACTION_BLOCK) }?;
        unsafe { rule.SetProtocol(NET_FW_IP_PROTOCOL_ANY.0) }?;
        unsafe { rules.Add(&rule) }?;
        Ok(())
    }

    fn unblock_generic(&self, filter_name: &str) -> Result<(), Box<dyn Error>> {
        let policy: INetFwPolicy2 =
            unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }?;
        let rules = unsafe { policy.Rules() }?;
        unsafe { rules.Remove(&BSTR::from(filter_name)) }?;
        Ok(())
    }

    fn is_blocked_generic(filter_name: &str) -> Result<bool, Box<dyn Error>> {
        let policy: INetFwPolicy2 =
            unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }?;
        let rules = unsafe { policy.Rules() }?;
        let rule_exists = unsafe { rules.Item(&BSTR::from(filter_name)) }.is_ok();
        Ok(rule_exists)
    }

    pub fn block_exe(&mut self, system_info: &mut SystemInfo) -> Result<(), Box<dyn Error>> {
        let Some(exe_path) = get_game_exe_path(system_info) else {
            self.blocked_status = BlockedStatus::Failed;
            return Ok(());
        };
        self.block_generic(Mode::EntireGame(exe_path.to_path_buf()))?;
        self.blocked_status = Self::is_exe_blocked()?.into();
        Ok(())
    }

    pub fn unblock_exe(&mut self) -> Result<(), Box<dyn Error>> {
        self.unblock_generic(FILTER_NAME_EXE)?;
        self.blocked_status = Self::is_exe_blocked()?.into();
        Ok(())
    }

    fn is_exe_blocked() -> Result<bool, Box<dyn Error>> {
        Self::is_blocked_generic(FILTER_NAME_EXE)
    }

    pub fn block_save_server(&mut self, save_server_ip: &str) -> Result<(), Box<dyn Error>> {
        self.block_generic(Mode::SaveServer(save_server_ip.to_owned()))?;
        self.blocked_status = Self::is_save_server_blocked()?.into();
        Ok(())
    }

    pub fn unblock_save_server(&mut self) -> Result<(), Box<dyn Error>> {
        self.unblock_generic(FILTER_NAME_SAVE_SERVER)?;
        self.blocked_status = Self::is_save_server_blocked()?.into();
        Ok(())
    }

    pub fn is_save_server_blocked() -> Result<bool, Box<dyn Error>> {
        Self::is_blocked_generic(FILTER_NAME_SAVE_SERVER)
    }

    pub fn reset_indicator_if_failed(&mut self) {
        if self.blocked_status == BlockedStatus::Failed && !self.counting {
            self.counting = true;
            self.timer = Instant::now();
        }
        if self.blocked_status == BlockedStatus::Failed
            && self.counting
            && self.timer.elapsed() >= INTERVAL
        {
            self.counting = false;
            self.blocked_status = Self::is_exe_blocked().unwrap().into();
        }
    }

    pub fn ensure_not_both_blocked_simultaneously(&mut self, block_method: BlockMethod) {
        match block_method {
            BlockMethod::EntireGame => {
                if Self::is_save_server_blocked().unwrap() {
                    // ignoring the return because if this is an error the user can just thug it out at that point
                    let _ = self.unblock_save_server();
                    self.blocked_status = Self::is_exe_blocked().unwrap().into();
                }
            }
            BlockMethod::SaveServer => {
                if Self::is_exe_blocked().unwrap() {
                    // ignoring the return because if this is an error the user can just thug it out at that point
                    let _ = self.unblock_exe();
                    self.blocked_status = Self::is_save_server_blocked().unwrap().into();
                }
            }
        }
    }
}

fn get_game_exe_path(system_info: &mut SystemInfo) -> Option<&Path> {
    system_info.refresh();
    system_info
        .processes()
        .iter()
        .find(|p| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
        .and_then(|p| p.exe())
}
