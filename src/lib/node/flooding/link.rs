use std::time::SystemTime;

#[derive(Debug, Clone)]
pub struct Link {
    pub addr: String,
    pub source: String,
    pub cost: u32,
    pub delay: u32,
    pub active: bool,
    pub times_down: u8,
    pub last_down: SystemTime,
}

impl Link {
    pub fn new(
        addr: String,
        source: String,
        cost: u32,
        delay: u32,
        active: bool,
        times_down: u8,
        last_down: SystemTime,
    ) -> Self {
        Self {
            addr,
            source,
            cost,
            delay,
            active,
            times_down,
            last_down,
        }
    }
}
