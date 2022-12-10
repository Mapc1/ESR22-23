use std::net::TcpStream;

use serde::{Deserialize, Serialize};

use crate::node::flooding::link::Link;
use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::packet::Packet;
use crate::node::packets::utils::get_peer_addr;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPacket {}

impl RequestPacket {
    pub fn from_bytes_packet_type(_bytes: Vec<u8>) -> RequestPacket {
        todo!()
    }
}

impl Packet for RequestPacket {
    fn handle(&self, mut stream: TcpStream, table: &mut RoutingTable) -> Result<bool, String> {
        return match table.handle_request_packet(get_peer_addr(stream)) {
            Ok(_) => Ok(false),
            Err(e) => Err(e),
        };
    }
}
