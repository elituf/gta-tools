use crate::util::{
    countdown::Countdown,
    firewall::{Firewall, RuleDirection, RuleMode, RuleProtocol},
    system_info::SystemInfo,
};
use std::{
    error::Error,
    time::{Duration, Instant},
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
    pub fn run_timers(&mut self, firewall: &Firewall) -> Result<(), Box<dyn Error>> {
        if self.disabled {
            self.countdown.count();
        } else {
            self.countdown.reset();
        }
        if self.interval.elapsed() >= INTERVAL {
            deactivate(firewall)?;
            self.disabled = false;
        }
        Ok(())
    }
}

pub fn activate(system_info: &mut SystemInfo, firewall: &Firewall) -> Result<bool, Box<dyn Error>> {
    let Some(exe_path) = system_info.get_game_exe_path() else {
        log::info!("wasn't able to find game exe");
        return Ok(false);
    };
    firewall.add(
        FILTER_NAME_EMPTY_SESSION_IN,
        RuleMode::Executable(exe_path.to_path_buf()),
        RuleDirection::In,
        RuleProtocol::Udp,
    )?;
    firewall.add(
        FILTER_NAME_EMPTY_SESSION_OUT,
        RuleMode::Executable(exe_path.to_path_buf()),
        RuleDirection::Out,
        RuleProtocol::Udp,
    )?;
    Ok(true)
}

pub fn deactivate(firewall: &Firewall) -> Result<(), Box<dyn Error>> {
    firewall.remove(FILTER_NAME_EMPTY_SESSION_IN)?;
    firewall.remove(FILTER_NAME_EMPTY_SESSION_OUT)?;
    Ok(())
}
