use crate::util::{
    consts::game::{EXE_ENHANCED, EXE_LEGACY},
    countdown::Countdown,
    system_info::SystemInfo,
};
use std::time::{Duration, Instant};
use windows::Win32::{
    Foundation::{HANDLE, NTSTATUS},
    System::Threading::{OpenProcess, PROCESS_SUSPEND_RESUME},
};

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
    pub fn run_timers(&mut self, game_handle: &mut HANDLE) {
        if self.disabled {
            self.countdown.count();
        } else {
            self.countdown.reset();
        }
        if self.interval.elapsed() >= INTERVAL {
            deactivate(game_handle);
            self.disabled = false;
        }
    }
}

#[link(name = "ntdll")]
unsafe extern "system" {
    unsafe fn NtSuspendProcess(ProcessHandle: HANDLE) -> NTSTATUS;
    unsafe fn NtResumeProcess(ProcessHandle: HANDLE) -> NTSTATUS;
}

fn get_gta_pid(system_info: &mut SystemInfo) -> Option<u32> {
    system_info.refresh();
    system_info
        .processes()
        .iter()
        .find(|p| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
        .map(|p| p.pid())
}

pub fn activate(game_handle: &mut HANDLE, system_info: &mut SystemInfo) -> bool {
    let Some(pid) = get_gta_pid(system_info) else {
        return false;
    };
    match unsafe { OpenProcess(PROCESS_SUSPEND_RESUME, false, pid) } {
        Ok(handle) => *game_handle = handle,
        Err(why) => {
            log::error!("failed to suspend game for empty session:\n{why}");
            return false;
        }
    }
    unsafe { NtSuspendProcess(*game_handle) }.unwrap();
    true
}

pub fn deactivate(game_handle: &mut HANDLE) {
    if !game_handle.is_invalid() {
        // ignoring the return because this function behaves very weirdly
        let _ = unsafe { NtResumeProcess(*game_handle) };
    }
}
