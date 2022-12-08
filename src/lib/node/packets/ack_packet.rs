use crate::node::flooding::link::Link;
use serde::{Deserialize, Serialize};
use std::net::TcpStream;

use crate::node::packets::packet::Packet;

#[derive(Serialize, Deserialize, Debug)]
pub struct AckPacket {}

impl AckPacket {
    pub fn from_bytes_packet_type(_bytes: Vec<u8>) -> AckPacket {
        todo!()
    }
}

impl Packet for AckPacket {
    fn handle(&self, mut stream: TcpStream, links: &mut Vec<Link>) {
        todo!()
    }
}
