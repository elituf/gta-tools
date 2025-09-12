use crate::util::{
    consts::game::{EXE_ENHANCED, EXE_LEGACY},
    log,
    system_info::SystemInfo,
};
use std::time::{Duration, Instant};

const INTERVAL: Duration = Duration::from_secs(3);

#[derive(Debug)]
pub struct ForceClose {
    pub button_text: String,
    timer: Instant,
    counting: bool,
    current_frame: bool,
}

impl Default for ForceClose {
    fn default() -> Self {
        Self {
            button_text: "Force close game".to_owned(),
            timer: Instant::now(),
            counting: false,
            current_frame: false,
        }
    }
}

impl ForceClose {
    pub fn prompt(&mut self, force_close_button_clicked: bool, system_info: &mut SystemInfo) {
        if force_close_button_clicked && !self.counting {
            self.button_text = "Are you sure?".to_owned();
            self.timer = Instant::now();
            self.counting = true;
            self.current_frame = true;
        }
        if self.counting && self.timer.elapsed() >= INTERVAL {
            self.reset();
        } else if force_close_button_clicked && !self.current_frame {
            activate(system_info);
            self.reset();
        }
        self.finish_current_frame();
    }

    fn reset(&mut self) {
        *self = Self::default();
    }

    const fn finish_current_frame(&mut self) {
        if self.current_frame {
            self.current_frame = false;
        }
    }
}

fn activate(system_info: &mut SystemInfo) {
    system_info.refresh();
    system_info
        .processes()
        .iter()
        .filter(|p| p.name() == EXE_ENHANCED || p.name() == EXE_LEGACY)
        .for_each(|p| {
            if !p.kill() {
                log::log(
                    log::LogLevel::Error,
                    "failed to force close game, probably due to access denied",
                );
            }
        });
    system_info.refresh();
}
