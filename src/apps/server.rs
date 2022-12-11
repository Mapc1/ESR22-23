use std::io::Write;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

use lib::node::packets::flood_packet::FloodPacket;
use lib::node::packets::utils;

fn main() {
    let neighbor_addr = "10.0.0.1";
    let neighbor_port = 1234;
    let timeout = Duration::from_secs(1);

    let my_addr = "10.0.0.0";

    let mut stream: TcpStream;

    loop {
        match TcpStream::connect(format!("{neighbor_addr}:{neighbor_port}")) {
            Ok(stream_res) => {
                stream = stream_res;
                break;
            }
            Err(_) => {
                println!("Error connecting to neighbor");
                thread::sleep(utils::TIMEOUT);
            }
        };
    }

    // Create Flood Packet and send to the neighbor_addr
    let packet = FloodPacket::new(my_addr.to_string(), 0, std::time::SystemTime::now());

    println!("Sending packet: {packet:#?}");

    match packet.to_bytes() {
        Ok(bytes) => {
            stream.write(&bytes).unwrap();
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
