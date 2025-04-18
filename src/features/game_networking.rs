#![allow(clippy::zombie_processes)]

use crate::util::consts::game::{EXE_ENHANCED, EXE_LEGACY};
use std::{os::windows::process::CommandExt, path::Path, process::Command};
use sysinfo::System;
use windows::Win32::System::Threading::CREATE_NO_WINDOW;

const FILTER_NAME: &str = "[GTA Tools] Block all traffic for GTA V";

fn get_game_exe_path(sysinfo: &mut System) -> Option<&Path> {
    sysinfo.refresh_all();
    if let Some((_, process)) = sysinfo
        .processes()
        .iter()
        .find(|(_, p)| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
    {
        process.exe()
    } else {
        None
    }
}

pub fn block_all(sysinfo: &mut System) {
    let Some(exe_path) = get_game_exe_path(sysinfo) else {
        return;
    };
    let exe_path = exe_path.display();
    Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "add",
            "rule",
            &format!("name={FILTER_NAME}"),
            "dir=out",
            "action=block",
            "protocol=ANY",
            &format!("program={exe_path}"),
        ])
        .creation_flags(CREATE_NO_WINDOW.0)
        .spawn()
        .unwrap();
    Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "add",
            "rule",
            &format!("name={FILTER_NAME}"),
            "dir=in",
            "action=block",
            "protocol=ANY",
            &format!("program={exe_path}"),
        ])
        .creation_flags(CREATE_NO_WINDOW.0)
        .spawn()
        .unwrap();
}

pub fn unblock_all() {
    Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "delete",
            "rule",
            &format!("name={FILTER_NAME}"),
        ])
        .creation_flags(CREATE_NO_WINDOW.0)
        .spawn()
        .unwrap();
}
