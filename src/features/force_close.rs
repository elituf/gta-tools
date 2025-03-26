use std::time::Instant;
use sysinfo::System;

const ENHANCED: &str = "GTA5_Enhanced.exe";
const LEGACY: &str = "GTA5.exe";

pub struct ForceClose {
    pub button_text: String,
    pub prompting: bool,
    pub interval: Instant,
}

impl Default for ForceClose {
    fn default() -> Self {
        Self {
            button_text: "Force close game".to_string(),
            prompting: false,
            interval: Instant::now(),
        }
    }
}

pub fn activate(sysinfo: &mut System) {
    sysinfo.refresh_all();
    sysinfo
        .processes()
        .iter()
        .filter(|(_, p)| p.name() == ENHANCED || p.name() == LEGACY)
        .for_each(|(_, p)| {
            p.kill();
        });
}
