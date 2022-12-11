use crate::node::flooding::link::Link;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};
use crate::node::flooding::routing_table::RoutingTable;

use crate::node::packets::packet::Packet;

#[derive(Serialize, Deserialize, Debug)]
pub struct RequestPacket {}

impl RequestPacket {
    pub fn from_bytes_packet_type(_bytes: Vec<u8>) -> RequestPacket {
        todo!()
    }
}

impl Packet for RequestPacket {
    fn handle(&self, mut stream: TcpStream, links: &mut Arc<RwLock<RoutingTable>>) -> Result<bool, String>{
        todo!()
    }
}
