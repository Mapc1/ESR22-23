use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::packet::Packet;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

#[derive(Serialize, Deserialize, Debug)]
pub struct AckPacket {}

impl AckPacket {
    pub fn from_bytes_packet_type(_bytes: Vec<u8>) -> AckPacket {
        todo!()
    }
}

impl Packet for AckPacket {
    fn handle(
        &self,
        _stream: TcpStream,
        _links: &mut Arc<RwLock<RoutingTable>>,
    ) -> Result<bool, String> {
        todo!()
    }

    fn get_type(&self) -> u8 {
        2
    }
}
