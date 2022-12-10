use crate::node::flooding::link::Link;
use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::ack_packet::AckPacket;
use crate::node::packets::flood_packet::FloodPacket;
use crate::node::packets::request_packet::RequestPacket;
use std::net::TcpStream;

#[derive(Debug)]
pub enum PacketType {
    Flood(FloodPacket),
    Request(RequestPacket), // Request for stream content
    Ack(AckPacket),
}

impl PacketType {
    pub fn from_u8(value: u8, bytes: Vec<u8>) -> Option<PacketType> {
        match value {
            0 => Some(PacketType::Flood(FloodPacket::from_bytes_packet_type(
                bytes,
            ))),
            1 => Some(PacketType::Request(RequestPacket::from_bytes_packet_type(
                bytes,
            ))),
            2 => Some(PacketType::Ack(AckPacket::from_bytes_packet_type(bytes))),
            _ => None,
        }
    }

    pub fn handle(&self, stream: TcpStream, table: &mut RoutingTable) -> Result<bool, String> {
        match self {
            PacketType::Flood(packet) => packet.handle(stream, table),
            PacketType::Request(packet) => packet.handle(stream, table),
            PacketType::Ack(packet) => packet.handle(stream, table),
        }
    }
}

pub trait Packet {
    fn handle(&self, stream: TcpStream, table: &mut RoutingTable) -> Result<bool, String>;
}

// [type u8, size  u16] -> Data[size] u8[size]
