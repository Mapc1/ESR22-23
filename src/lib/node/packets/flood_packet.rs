use std::time::SystemTime;

pub struct FloodPacket {
    source: String,
    cost: u32,
    timestamp: SystemTime,
}
impl FloodPacket {
    pub fn new(source: &String, cost: u32, timestamp: &SystemTime) -> Self {
        Self {
            source: source.to_owned(),
            cost,
            timestamp: timestamp.to_owned()
        }
    }

    pub fn get_bytes(&self) -> Vec<u8> {
        let bytes: Vec<u8> = Vec::new();

        for byte in self.source.bytes() {
            bytes.push(byte);
        }
    }
}
