use crate::util::{
    consts::game::{EXE_ENHANCED, EXE_LEGACY},
    system_info::SystemInfo,
};
use std::{
    path::Path,
    time::{Duration, Instant},
};
use strum::{Display, EnumIter};
use windows::{
    Win32::{
        NetworkManagement::WindowsFirewall::{
            INetFwPolicy2, INetFwRule, NET_FW_ACTION_BLOCK, NET_FW_IP_PROTOCOL_ANY,
            NET_FW_RULE_DIR_IN, NET_FW_RULE_DIR_OUT, NetFwPolicy2, NetFwRule,
        },
        System::Com::{
            CLSCTX_INPROC_SERVER, COINIT_MULTITHREADED, CoCreateInstance, CoInitializeEx,
            CoUninitialize,
        },
    },
    core::BSTR,
};

const FILTER_NAME_IN: &str = "[GTA Tools] Block all inbound traffic for GTA V";
const FILTER_NAME_OUT: &str = "[GTA Tools] Block all outbound traffic for GTA V";

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
            blocked_status: Self::is_blocked().into(),
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

impl GameNetworking {
    pub fn block_all(&mut self, system_info: &mut SystemInfo) {
        let Some(exe_path) = get_game_exe_path(system_info) else {
            self.blocked_status = BlockedStatus::Failed;
            return;
        };
        let policy: INetFwPolicy2 =
            unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }.unwrap();
        let rules = unsafe { policy.Rules() }.unwrap();
        let exe_path = BSTR::from(exe_path.to_string_lossy().to_string());
        for filter in [
            (FILTER_NAME_IN, NET_FW_RULE_DIR_IN),
            (FILTER_NAME_OUT, NET_FW_RULE_DIR_OUT),
        ] {
            unsafe { rules.Remove(&BSTR::from(filter.0)) }.unwrap();
            let rule: INetFwRule =
                unsafe { CoCreateInstance(&NetFwRule, None, CLSCTX_INPROC_SERVER) }.unwrap();
            unsafe { rule.SetName(&BSTR::from(filter.0)) }.unwrap();
            unsafe { rule.SetApplicationName(&exe_path) }.unwrap();
            unsafe { rule.SetDirection(filter.1) }.unwrap();
            unsafe { rule.SetEnabled(true.into()) }.unwrap();
            unsafe { rule.SetAction(NET_FW_ACTION_BLOCK) }.unwrap();
            unsafe { rule.SetProtocol(NET_FW_IP_PROTOCOL_ANY.0) }.unwrap();
            unsafe { rules.Add(&rule) }.unwrap();
        }
        self.blocked_status = Self::is_blocked().into();
    }

    pub fn unblock_all(&mut self) {
        let policy: INetFwPolicy2 =
            unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }.unwrap();
        let rules = unsafe { policy.Rules() }.unwrap();
        unsafe { rules.Remove(&BSTR::from(FILTER_NAME_IN)) }.unwrap();
        unsafe { rules.Remove(&BSTR::from(FILTER_NAME_OUT)) }.unwrap();
        self.blocked_status = Self::is_blocked().into();
    }

    fn is_blocked() -> bool {
        let policy: INetFwPolicy2 =
            unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }.unwrap();
        let rules = unsafe { policy.Rules() }.unwrap();
        let in_rule_exists = unsafe { rules.Item(&BSTR::from(FILTER_NAME_IN)) }.is_ok();
        let out_rule_exists = unsafe { rules.Item(&BSTR::from(FILTER_NAME_OUT)) }.is_ok();
        in_rule_exists || out_rule_exists
    }

    pub fn if_failed_return_to_boolean(&mut self) {
        if self.blocked_status == BlockedStatus::Failed && !self.counting {
            self.counting = true;
            self.timer = Instant::now();
        }
        if self.blocked_status == BlockedStatus::Failed
            && self.counting
            && self.timer.elapsed() >= INTERVAL
        {
            self.counting = false;
            self.blocked_status = Self::is_blocked().into();
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
