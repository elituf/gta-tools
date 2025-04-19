use crate::util::{
    consts::game::{EXE_ENHANCED, EXE_LEGACY},
    countdown::Countdown,
};
use std::time::{Duration, Instant};
use sysinfo::System;
use windows::Win32::{
    Foundation::{CloseHandle, HANDLE, NTSTATUS},
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

fn get_gta_pid(sysinfo: &mut System) -> u32 {
    sysinfo.refresh_all();
    if let Some((pid, _)) = sysinfo
        .processes()
        .iter()
        .find(|(_, p)| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
    {
        return pid.as_u32();
    }
    u32::MAX
}

pub fn activate(game_handle: &mut HANDLE, sysinfo: &mut System) {
    let pid = get_gta_pid(sysinfo);
    if pid == u32::MAX {
        return;
    }
    unsafe {
        *game_handle = OpenProcess(PROCESS_SUSPEND_RESUME, false, pid).unwrap();
        let _ = NtSuspendProcess(*game_handle);
    }
}

pub fn deactivate(game_handle: &mut HANDLE) {
    unsafe {
        if !game_handle.is_invalid() {
            let _ = NtResumeProcess(*game_handle);
            let _ = CloseHandle(*game_handle);
        }
    }
}
