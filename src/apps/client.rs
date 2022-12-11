use std::{net::TcpStream, io::Write};

use lib::node::packets::request_packet::RequestPacket;

fn main() {
    let mut tcp = TcpStream::connect("10.0.5.2:1234").unwrap();
    let bytes = RequestPacket::new().to_bytes();

    tcp.write(&bytes[..]).unwrap();
}