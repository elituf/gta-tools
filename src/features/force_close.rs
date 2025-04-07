use crate::util::consts::{ENHANCED, LEGACY};
use std::time::Instant;
use sysinfo::System;

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

impl ForceClose {
    pub fn prompting(&mut self) {
        self.button_text = "Are you sure?".to_string();
        self.prompting = true;
        self.interval = Instant::now();
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
    sysinfo.refresh_all();
}
