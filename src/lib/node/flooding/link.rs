use std::time::{Duration, SystemTime};

#[derive(Debug, Clone)]
pub struct Link {
    pub addr: String,
    pub source: String,
    pub jumps: u32,
    pub delay: u64,
    pub active: bool,
    pub times_down: u8,
    pub last_down: SystemTime,
    pub has_clients: bool,
}

impl Link {
    pub fn new(
        addr: String,
        source: String,
        jumps: u32,
        delay: u64,
        active: bool,
        times_down: u8,
        last_down: SystemTime,
        has_clients: bool
    ) -> Self {
        Self {
            addr,
            source,
            jumps,
            delay,
            active,
            times_down,
            last_down,
            has_clients
        }
    }

    pub fn new_default<S: Into<String>>(addr: S) -> Self {
        Self {
            addr: addr.into(),
            source: "empty".to_string(),
            jumps: u32::MAX,
            delay: u64::MAX,
            active: false,
            times_down: 0,
            last_down: SystemTime::UNIX_EPOCH,
            has_clients: false
        }
    }
}
