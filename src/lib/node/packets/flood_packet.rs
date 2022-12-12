use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::node::flooding::link::Link;
use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::packet::Packet;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FloodPacket {
    pub source: String,
    pub jumps: u32,
    pub timestamp: u64,
}

impl FloodPacket {
    pub fn new(source: String, cost: u32, timestamp: SystemTime) -> Self {
        Self {
            source,
            jumps: cost,
            timestamp: timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }

    pub fn from_bytes_packet_type(bytes: Vec<u8>) -> FloodPacket {
        let mut des = rmp_serde::Deserializer::new(&bytes[..]);

        Deserialize::deserialize(&mut des).expect("ola")
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let pack_type: u8 = 0; // FIXME
        let mut data = match rmp_serde::to_vec(self) {
            Ok(vec) => vec,
            Err(err) => return Err(err.to_string()),
        };
        let mut size = (data.len() as u16 + 3).to_be_bytes().to_vec();

        let mut pack_data = vec![pack_type];
        pack_data.append(&mut size);
        pack_data.append(&mut data);

        Ok(pack_data)
    }

    pub fn from_link(link: &Link) -> Self {
        Self {
            source: link.addr.to_string(),
            jumps: link.jumps + 1,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        }
    }
}

impl Packet for FloodPacket {
    fn handle(
        &self,
        stream: TcpStream,
        table: &mut Arc<RwLock<RoutingTable>>,
    ) -> Result<bool, String> {
        let peer_addr = stream.peer_addr().unwrap().ip().to_string();
        let changed = table
            .write()
            .unwrap()
            .handle_flood_packet(peer_addr, self)?;

        Ok(changed)
    }

    fn get_type(&self) -> u8 {
        0
    }
}
