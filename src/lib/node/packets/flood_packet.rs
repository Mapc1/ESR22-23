use std::net::TcpStream;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::node::flooding::link::Link;
use crate::node::packets::packet::Packet;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FloodPacket {
    source: String,
    cost: u32,
    timestamp: SystemTime,
}

impl FloodPacket {
    pub fn new(source: String, cost: u32, timestamp: SystemTime) -> Self {
        Self {
            source,
            cost,
            timestamp,
        }
    }

    pub fn new_from_bytes(_bytes: Vec<u8>) -> Self {
        Self {
            source: "".to_string(),
            cost: 0,
            timestamp: SystemTime::now(),
        }
    }

    pub fn from_bytes_packet_type(bytes: Vec<u8>) -> FloodPacket {
        FloodPacket::new_from_bytes(bytes)
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        match rmp_serde::to_vec(self) {
            Ok(vec) => Ok(vec),
            Err(err) => Err(err.to_string()),
        }
    }
}

impl Packet for FloodPacket {
    fn handle(&self, mut stream: TcpStream, links: &Vec<Link>) {
        let mut i = 0;
        let link = loop {
            let l = links.get(i).unwrap();
            if l.source == self.source {
                break l;
            }
            i += 1;
        };

        println!("{}", link.source);
    }
}
