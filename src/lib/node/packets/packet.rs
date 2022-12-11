use crate::node::packets::{
    ack_packet::AckPacket,
    flood_packet::FloodPacket,
    request_packet::RequestPacket,
    stream_packet::StreamPacket,
};
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};
use crate::node::flooding::routing_table::RoutingTable;

use super::refuse_packet::RefusePacket;


#[derive(Debug)]
pub enum PacketType {
    Flood(FloodPacket),
    Request(RequestPacket), // Request for stream content
    Ack(AckPacket),
    Stream(StreamPacket),
    Refuse(RefusePacket),
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
            4 => Some(PacketType::Refuse(RefusePacket::from_bytes_packet_type(
                bytes,
            ))),
            _ => None,
        }
    }

    pub fn handle(&mut self, stream: TcpStream, table: &mut Arc<RwLock<RoutingTable>>) -> Result<bool, String> {
        match self {
            PacketType::Flood(packet) => packet.handle(stream, table),
            PacketType::Request(packet) => packet.handle(stream, table),
            PacketType::Ack(packet) => packet.handle(stream, table),
            PacketType::Stream(packet) => Err("FIXME".to_string()),
            PacketType::Refuse(packet) => packet.handle(stream, table),
        }
    }
}

impl dyn Packet {
    fn serialize(packet: FloodPacket) -> Result<Vec<u8>, String> {
        let packet_type: u8 = packet.get_type(); // FIXME
        let mut data = match rmp_serde::to_vec(&packet) {
            Ok(vec) => vec,
            Err(err) => return Err(err.to_string()),
        };
        let mut size = (data.len() as u16 + 3).to_be_bytes().to_vec();

        let mut serialized_packet = vec![packet_type];
        serialized_packet.append(&mut size);
        serialized_packet.append(&mut data);

        Ok(serialized_packet)
    }

    pub fn send_packet(packet: Vec<u8>, peer_addr: String) -> Result<(), String> {
        let mut stream = TcpStream::connect(peer_addr).expect("Failed to connect");
        stream.write_all(&packet).expect("Failed to write");
        Ok(())
    }
}

pub trait Packet {
    fn handle(&self, stream: TcpStream, table: &mut Arc<RwLock<RoutingTable>>) -> Result<bool, String>;
    fn get_type(&self) -> u8;
}

// [type u8, size  u16] -> Data[size] u8[size]
