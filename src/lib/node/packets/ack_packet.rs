use std::net::TcpStream;

use serde::{Deserialize, Serialize};

use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::packet::Packet;

#[derive(Serialize, Deserialize, Debug)]
pub struct AckPacket {}

impl AckPacket {
    pub fn from_bytes_packet_type(_bytes: Vec<u8>) -> AckPacket {
        todo!()
    }
}

impl Packet for AckPacket {
    fn get_type(&self) -> u8 {
        2
    }

    fn handle(&self, _stream: TcpStream, _links: &mut RoutingTable) -> Result<bool, String> {
        todo!()
    }
}
