use crate::util::consts::{ENHANCED, LEGACY};
use std::{path::Path, process::Command};
use sysinfo::System;

const FILTER_NAME: &str = "[GTA Tools] Block all traffic for GTA V";

fn get_game_exe_path(sysinfo: &mut System) -> Option<&Path> {
    sysinfo.refresh_all();
    if let Some((_, process)) = sysinfo
        .processes()
        .iter()
        .find(|(_, p)| p.name() == ENHANCED || p.name() == LEGACY)
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
    let exe_path = exe_path.display().to_string();
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
        .spawn()
        .unwrap();
    Command::new("netsh")
        .args([
            "advfirewall",
            "firewall",
            "delete",
            "rule",
            &format!("name={FILTER_NAME}"),
        ])
        .spawn()
        .unwrap();
}
