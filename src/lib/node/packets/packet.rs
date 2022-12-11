use crate::node::flooding::link::Link;
use crate::node::packets::{
    ack_packet::AckPacket,
    flood_packet::FloodPacket,
    request_packet::RequestPacket,
    stream_packet::StreamPacket,
};
use std::net::TcpStream;
use std::sync::{Arc, Mutex, RwLock};
use crate::node::flooding::routing_table::RoutingTable;


#[derive(Debug)]
pub enum PacketType {
    Flood(FloodPacket),
    Request(RequestPacket), // Request for stream content
    Ack(AckPacket),
    Stream(StreamPacket),
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
            3 => Some(PacketType::Stream(StreamPacket::from_bytes_packet_type(bytes))),
            _ => None,
        }
    }

    pub fn handle(&mut self, stream: TcpStream, table: &mut Arc<RwLock<RoutingTable>>) -> Result<bool, String> {
        match self {
            PacketType::Flood(packet) => packet.handle(stream, table),
            PacketType::Request(packet) => packet.handle(stream, table),
            PacketType::Ack(packet) => packet.handle(stream, table),
            PacketType::Stream(packet) => Err("FIXME".to_string()),
        }
    }
}

pub trait Packet {
    fn handle(&self, stream: TcpStream, table: &mut Arc<RwLock<RoutingTable>>) -> Result<bool, String>;
}

// [type, size] -> Data[size]
