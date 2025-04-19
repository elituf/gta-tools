use crate::util::consts::game::{EXE_ENHANCED, EXE_LEGACY};
use std::time::{Duration, Instant};
use sysinfo::System;

const INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug)]
pub struct ForceClose {
    pub button_text: String,
    prompting: bool,
    interval: Instant,
    current_frame: bool,
}

impl Default for ForceClose {
    fn default() -> Self {
        Self {
            button_text: "Force close game".to_owned(),
            prompting: false,
            interval: Instant::now(),
            current_frame: false,
        }
    }
}

impl ForceClose {
    pub fn prompt(&mut self, force_close_button_clicked: bool, sysinfo: &mut System) {
        if force_close_button_clicked && !self.prompting {
            *self = Self {
                button_text: "Are you sure?".to_owned(),
                prompting: true,
                interval: Instant::now(),
                current_frame: true,
            }
        }
        if self.prompting && self.interval.elapsed() <= INTERVAL {
            if force_close_button_clicked && !self.current_frame {
                activate(sysinfo);
                *self = Self::default();
            }
        } else {
            *self = Self::default();
        }
        if self.current_frame {
            self.current_frame = false;
        }
    }
}

fn activate(sysinfo: &mut System) {
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
