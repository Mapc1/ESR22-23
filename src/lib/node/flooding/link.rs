use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct Link {
    pub addr: String,
    pub source: String,
    pub jumps: u32,
    pub delay: Duration,
    pub active: bool,
    pub times_down: u8,
    pub last_down: SystemTime,
}

impl Link {
    pub fn new(
        addr: String,
        source: String,
        jumps: u32,
        delay: Duration,
        active: bool,
        times_down: u8,
        last_down: SystemTime,
    ) -> Self {
        Self {
            addr,
            source,
            jumps,
            delay,
            active,
            times_down,
            last_down,
        }
    }
}
