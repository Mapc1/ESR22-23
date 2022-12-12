use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, RwLock};

use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::flood_packet::FloodPacket;
use crate::node::packets::packet::PacketType;
use crate::node::packets::stream_packet::StreamPacket;
use crate::types::networking::Addr;

const LISTENER_IP: &Addr = "0.0.0.0";
pub const LISTENER_PORT: u16 = 1234;
const MAX_BUFF_SIZE: usize = 65000;

pub fn udp_listener(table: &mut Arc<RwLock<RoutingTable>>) -> Result<(), String> {
    let socket = match UdpSocket::bind(format!("{LISTENER_IP}:{LISTENER_PORT}")) {
        Ok(socket) => socket,
        Err(err) => return Err(err.to_string()),
    };

    let mut buf: [u8; MAX_BUFF_SIZE] = [0; MAX_BUFF_SIZE];
    loop {
        let (_, addr) = socket.recv_from(&mut buf).unwrap();

        let pack_size = u16::from_be_bytes(buf[0..2].try_into().unwrap());
        println!("{pack_size} | {}", buf[5]);
        let mut pack = StreamPacket::from_bytes_packet_type(buf[2..pack_size as usize].to_vec());

        pack.handle(&socket, addr.ip().to_string(), table)?;
    }
}

pub fn listener(table: &mut Arc<RwLock<RoutingTable>>) -> Result<(), String> {
    let listener = match TcpListener::bind(format!("{LISTENER_IP}:{LISTENER_PORT}")) {
        Ok(listener) => listener,
        Err(_) => return Err("Error binding listener".to_string()),
    };

    // accept connections and respond to each packet sent
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_packet(stream, table) {
                Ok(_) => (),
                Err(err) => return Err(err),
            },
            Err(_) => return Err("Something went wrong with the connection".to_string()),
        }
    }
    Ok(())
}

pub fn handle_packet(
    mut stream: TcpStream,
    table: &mut Arc<RwLock<RoutingTable>>,
) -> Result<(), String> {
    // Packet -> [type: u8, size:u16] 3 bytes -> Data[size]
    let mut buffer = [0; 1500];

    let _size = match stream.read(&mut buffer) {
        Ok(size) => size,
        Err(_) => return Err("Error reading from stream".to_string()),
    };

    let packet_size = u16::from_be_bytes(buffer[1..3].try_into().unwrap());

    let mut packet = match PacketType::from_u8(buffer[0], buffer[3..packet_size as usize].to_vec())
    {
        Some(packet) => packet,
        None => return Err("Invalid packet type".to_string()),
    };

    let changed = packet.handle(stream, table)?;

    if changed {
        let lock = table.write().unwrap();
        let flood_pack = FloodPacket::from_link(&lock.closest_link);

        for l in lock.links.iter() {
            if l.addr == lock.closest_link.addr {
                continue;
            }

            //println!("Sending flood to {}", l.addr);
            let mut stream =
                TcpStream::connect(format!("{}:{}", l.addr.clone(), LISTENER_PORT)).unwrap();
            stream
                .write(flood_pack.to_bytes().unwrap().as_ref())
                .unwrap();
        }
    }

    // ...

    Ok(())
}
