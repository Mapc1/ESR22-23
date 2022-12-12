use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::packet::Packet;
use crate::node::packets::utils::get_peer_addr;
use crate::node::threads::listener::LISTENER_PORT;

use super::utils::connect;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPacket {}

impl RequestPacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.get_type(), 3]
    }

    pub fn new() -> Self {
        Self {}
    }

    pub fn from_bytes_packet_type(_bytes: Vec<u8>) -> RequestPacket {
        Self::new()
    }
}

impl Packet for RequestPacket {
    fn get_type(&self) -> u8 {
        1
    }

    fn handle(
        &self,
        stream: TcpStream,
        table: &mut Arc<RwLock<RoutingTable>>,
    ) -> Result<bool, String> {
        let mut lock = table.write().unwrap();
        match lock.handle_request_packet(get_peer_addr(&stream)) {
            Ok(_) => Ok(false),
            Err(e) => Err(e),
        }
        .unwrap();
        if lock.num_stream_connections == 1 {
            let mut back_stream = match connect(&lock.closest_link.addr, LISTENER_PORT) {
                Ok(stream) => stream,
                Err(e) => return Err(e),
            };
            return match back_stream.write(&self.to_bytes()) {
                Ok(_) => Ok(true),
                Err(e) => Err(e.to_string()),
            };
        }

        Ok(false)
    }
}
