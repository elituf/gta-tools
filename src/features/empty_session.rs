use crate::util::{countdown::Countdown, system_info::SystemInfo};
use std::{
    error::Error,
    time::{Duration, Instant},
};
use windows::{
    Win32::{
        NetworkManagement::WindowsFirewall::{
            INetFwPolicy2, INetFwRule, NET_FW_ACTION_BLOCK, NET_FW_IP_PROTOCOL_UDP,
            NET_FW_RULE_DIR_IN, NET_FW_RULE_DIR_OUT, NetFwPolicy2, NetFwRule,
        },
        System::Com::{CLSCTX_INPROC_SERVER, CoCreateInstance},
    },
    core::BSTR,
};

const FILTER_NAME_EMPTY_SESSION_IN: &str = "[GTA Tools] Block inbound UDP traffic for all of GTA V";
const FILTER_NAME_EMPTY_SESSION_OUT: &str =
    "[GTA Tools] Block outbound UDP traffic for all of GTA V";

const INTERVAL: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub struct EmptySession {
    pub disabled: bool,
    pub interval: Instant,
    pub countdown: Countdown,
}

impl Default for EmptySession {
    fn default() -> Self {
        Self {
            disabled: false,
            interval: Instant::now(),
            countdown: Countdown::new(INTERVAL.as_secs()),
        }
    }
}

impl EmptySession {
    pub fn run_timers(&mut self) -> Result<(), Box<dyn Error>> {
        if self.disabled {
            self.countdown.count();
        } else {
            self.countdown.reset();
        }
        if self.interval.elapsed() >= INTERVAL {
            deactivate()?;
            self.disabled = false;
        }
        Ok(())
    }
}

pub fn activate(system_info: &mut SystemInfo) -> Result<bool, Box<dyn Error>> {
    let Some(exe_path) = system_info.get_game_exe_path() else {
        log::info!("wasn't able to find game exe");
        return Ok(false);
    };
    for (direction, filter_name) in [
        (NET_FW_RULE_DIR_IN, FILTER_NAME_EMPTY_SESSION_IN),
        (NET_FW_RULE_DIR_OUT, FILTER_NAME_EMPTY_SESSION_OUT),
    ] {
        let policy: INetFwPolicy2 =
            unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }?;
        let rules = unsafe { policy.Rules() }?;
        unsafe { rules.Remove(&BSTR::from(filter_name)) }?;
        let rule: INetFwRule = unsafe { CoCreateInstance(&NetFwRule, None, CLSCTX_INPROC_SERVER) }?;
        unsafe { rule.SetName(&BSTR::from(filter_name)) }?;
        unsafe { rule.SetApplicationName(&BSTR::from(exe_path.to_string_lossy().to_string())) }?;
        unsafe { rule.SetDirection(direction) }?;
        unsafe { rule.SetEnabled(true.into()) }?;
        unsafe { rule.SetAction(NET_FW_ACTION_BLOCK) }?;
        unsafe { rule.SetProtocol(NET_FW_IP_PROTOCOL_UDP.0) }?;
        unsafe { rules.Add(&rule) }?;
    }
    Ok(true)
}

pub fn deactivate() -> Result<(), Box<dyn Error>> {
    let policy: INetFwPolicy2 =
        unsafe { CoCreateInstance(&NetFwPolicy2, None, CLSCTX_INPROC_SERVER) }?;
    let rules = unsafe { policy.Rules() }?;
    for filter_name in [FILTER_NAME_EMPTY_SESSION_IN, FILTER_NAME_EMPTY_SESSION_OUT] {
        unsafe { rules.Remove(&BSTR::from(filter_name)) }?;
    }
    Ok(())
}
