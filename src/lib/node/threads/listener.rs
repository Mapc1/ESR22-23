use std::io::Read;
use std::net::{TcpListener, TcpStream};

use crate::node::flooding::link::Link;
use crate::node::packets::packet::PacketType;
use crate::types::networking::Addr;

const LISTENER_IP: &Addr = "0.0.0.0";
const LISTENER_PORT: u16 = 1234;

pub fn listener() -> Result<(), String> {
    let links: Vec<Link> = Vec::new();

    let listener = match TcpListener::bind(format!("{LISTENER_IP}:{LISTENER_PORT}")) {
        Ok(listener) => listener,
        Err(_) => return Err("Error binding listener".to_string()),
    };

    // accept connections and respond to each packet sent
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => match handle_packet(stream, &links) {
                Ok(_) => (),
                Err(err) => return Err(err),
            },
            Err(_) => return Err("Something went wrong with the connection".to_string()),
        }
    }
    Ok(())
}

pub fn handle_packet(mut stream: TcpStream, links: &Vec<Link>) -> Result<(), String> {
    // Packet -> [type: u8, size:u16] 3 bytes -> Data[size]
    let mut buffer = [0; 1500];

    let _size = match stream.read(&mut buffer) {
        Ok(size) => size,
        Err(_) => return Err("Error reading from stream".to_string()),
    };

    let packet_size = usize::from_be_bytes(buffer[1..3].try_into().unwrap());

    let packet = match PacketType::from_u8(buffer[0], buffer[4..packet_size].to_vec()) {
        Some(packet) => packet,
        None => return Err("Invalid packet type".to_string()),
    };

    println!("Packet: {packet:?}");

    packet.handle(stream, links);

    // ...

    Ok(())
}
