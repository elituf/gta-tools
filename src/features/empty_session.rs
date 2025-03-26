use crate::gui::App;
use std::time::{Duration, Instant};
use sysinfo::System;
use windows::Win32::{
    Foundation::{HANDLE, NTSTATUS},
    System::Threading::{OpenProcess, PROCESS_SUSPEND_RESUME},
};

pub const INTERVAL: Duration = Duration::from_secs(10);
const ENHANCED: &str = "GTA5_Enhanced.exe";
const LEGACY: &str = "GTA5.exe";

pub struct EmptySession {
    pub enabled: bool,
    pub interval: Instant,
}

impl Default for EmptySession {
    fn default() -> Self {
        Self {
            enabled: true,
            interval: Instant::now(),
        }
    }
}

#[link(name = "ntdll")]
unsafe extern "system" {
    pub unsafe fn NtSuspendProcess(ProcessHandle: HANDLE) -> NTSTATUS;
    pub unsafe fn NtResumeProcess(ProcessHandle: HANDLE) -> NTSTATUS;
}

fn get_gta_pid(sysinfo: &mut System) -> u32 {
    sysinfo.refresh_all();
    if let Some((pid, _)) = sysinfo
        .processes()
        .iter()
        .find(|(_, p)| p.name() == ENHANCED || p.name() == LEGACY)
    {
        return pid.as_u32();
    };
    u32::MAX
}

pub fn activate(app: &mut App) {
    let pid = get_gta_pid(&mut app.sysinfo);
    if pid == u32::MAX {
        return;
    }
    unsafe {
        app.game_handle = OpenProcess(PROCESS_SUSPEND_RESUME, false, pid).unwrap();
        let _ = NtSuspendProcess(app.game_handle);
    }
}

pub fn deactivate(app: &mut App) {
    unsafe {
        let _ = NtResumeProcess(app.game_handle);
    }
}
