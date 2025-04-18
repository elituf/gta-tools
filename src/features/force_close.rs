use crate::util::consts::game::{EXE_ENHANCED, EXE_LEGACY};
use std::time::Instant;
use sysinfo::System;

#[derive(Debug)]
pub struct ForceClose {
    pub button_text: String,
    pub prompting: bool,
    pub interval: Instant,
}

impl Default for ForceClose {
    fn default() -> Self {
        Self {
            button_text: "Force close game".to_owned(),
            prompting: false,
            interval: Instant::now(),
        }
    }
}

impl ForceClose {
    pub fn prompting(&mut self) {
        self.button_text = "Are you sure?".to_owned();
        self.prompting = true;
        self.interval = Instant::now();
    }
}

pub fn activate(sysinfo: &mut System) {
    sysinfo.refresh_all();
    sysinfo
        .processes()
        .iter()
        .filter(|(_, p)| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
        .for_each(|(_, p)| {
            p.kill();
        });
    sysinfo.refresh_all();
}
