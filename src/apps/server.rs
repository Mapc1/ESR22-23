use std::io::Write;
use std::net::{TcpStream, UdpSocket};
use std::thread;
use std::time::Duration;

use lib::node::packets::flood_packet::FloodPacket;
use lib::node::packets::utils;

use lib::node::packets::stream_packet::StreamPacket;

fn main() {
    let neighbor_addr = "10.0.0.1";
    let neighbor_port = 1234;

    let my_addr = "10.0.0.0";

    let full_neighbour_addr = format!("{neighbor_addr}:{neighbor_port}");
    let mut stream = loop {
        match TcpStream::connect(full_neighbour_addr.clone()) {
            Ok(stream_res) => {
                break stream_res;
            }
            Err(_) => {
                println!("Error connecting to neighbor");
                thread::sleep(utils::TIMEOUT);
            }
        };
    };

    // Create Flood Packet and send to the neighbor_addr
    let packet = FloodPacket::new(my_addr.to_string(), 0, std::time::SystemTime::now());

    println!("Sending packet: {packet:#?}");

    match packet.to_bytes() {
        Ok(bytes) => {
            stream.write(&bytes).unwrap();
        }
        Err(err) => {
            println!("{}", err);
            return;
        }
    }

    println!("Sending stream");
    let mut stream_pack = StreamPacket::new("vagina".as_bytes().to_vec());
    thread::sleep(Duration::from_secs(2));
    let udp_stream = UdpSocket::bind(format!("0.0.0.0:1234")).unwrap();
    match stream_pack.to_bytes() {
        Ok(bytes) => {
            udp_stream.send_to(&bytes[..], full_neighbour_addr).unwrap();
        }
        Err(err) =>{
            println!("{}", err.to_string());
            return;
        }
    }

}
