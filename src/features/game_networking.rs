use crate::{gui::settings::BlockMethod, util::system_info::SystemInfo};
use std::{error::Error, path::PathBuf};
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

#[derive(Debug)]
pub struct GameNetworking {
    com_initialized: bool,
    pub blocked: bool,
}

impl Default for GameNetworking {
    fn default() -> Self {
        Self {
            blocked: if Self::is_save_server_blocked().unwrap() {
                Self::is_save_server_blocked().unwrap()
            } else {
                Self::is_exe_blocked().unwrap()
            },
            com_initialized: unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.is_ok(),
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
    fn block_generic(mode: Mode) -> Result<(), Box<dyn Error>> {
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

    fn unblock_generic(filter_name: &str) -> Result<(), Box<dyn Error>> {
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
            return Ok(());
        };
        Self::block_generic(Mode::EntireGame(exe_path.to_path_buf()))?;
        self.blocked = Self::is_exe_blocked()?;
        Ok(())
    }

    pub fn unblock_exe(&mut self) -> Result<(), Box<dyn Error>> {
        Self::unblock_generic(FILTER_NAME_EXE)?;
        self.blocked = Self::is_exe_blocked()?;
        Ok(())
    }

    fn is_exe_blocked() -> Result<bool, Box<dyn Error>> {
        Self::is_blocked_generic(FILTER_NAME_EXE)
    }

    pub fn block_save_server(&mut self, save_server_ip: &str) -> Result<(), Box<dyn Error>> {
        Self::block_generic(Mode::SaveServer(save_server_ip.to_owned()))?;
        self.blocked = Self::is_save_server_blocked()?;
        Ok(())
    }

    pub fn unblock_save_server(&mut self) -> Result<(), Box<dyn Error>> {
        Self::unblock_generic(FILTER_NAME_SAVE_SERVER)?;
        self.blocked = Self::is_save_server_blocked()?;
        Ok(())
    }

    pub fn is_save_server_blocked() -> Result<bool, Box<dyn Error>> {
        Self::is_blocked_generic(FILTER_NAME_SAVE_SERVER)
    }

    pub fn ensure_block_exclusivity(
        &mut self,
        block_method: BlockMethod,
    ) -> Result<(), Box<dyn Error>> {
        match block_method {
            BlockMethod::EntireGame => {
                if Self::is_save_server_blocked()? {
                    self.unblock_save_server()?;
                    self.blocked = Self::is_exe_blocked()?;
                }
            }
            BlockMethod::SaveServer => {
                if Self::is_exe_blocked()? {
                    self.unblock_exe()?;
                    self.blocked = Self::is_save_server_blocked()?;
                }
            }
        }
        Ok(())
    }
}
