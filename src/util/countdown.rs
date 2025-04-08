use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Countdown {
    pub i: u64,
    pub i_original: u64,
    pub i_string: String,
    pub interval: Instant,
    pub first_count: bool,
}

impl Countdown {
    pub fn new(i: u64) -> Self {
        Self {
            i,
            i_original: i,
            i_string: String::new(),
            interval: Instant::now(),
            first_count: true,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.i_original);
    }

    pub fn count(&mut self) {
        if self.first_count {
            self.interval = Instant::now();
            self.first_count = false;
        }
        if self.interval.elapsed() >= Duration::from_secs(1) {
            self.i -= 1;
            self.interval = Instant::now();
        }
        self.i_string = self.i.to_string();
        if self.i == 0 {
            self.reset();
        };
    }
}
