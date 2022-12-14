use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, UdpSocket};
use std::sync::{Arc, RwLock};
use std::thread;

use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::flood_packet::FloodPacket;
use crate::node::packets::packet::PacketType;
use crate::node::packets::stream_packet::StreamPacket;
use crate::node::packets::utils::get_peer_addr;
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
        //println!("{pack_size} | {}", buf[5]);
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
        let tcp_stream = match stream {
            Ok(stream) => stream,
            Err(_) => return Err("Something went wrong with the connection".to_string()),
        };

        println!("Accepted connection from {}", get_peer_addr(&tcp_stream)?);

        let mut table_cloned = table.clone();
        thread::spawn(
            move || match handle_connection(tcp_stream, &mut table_cloned) {
                Ok(_) => (),
                Err(err) => println!("{}", err),
            },
        );
    }

    Ok(())
}

pub fn handle_connection(
    stream: TcpStream,
    table: &mut Arc<RwLock<RoutingTable>>,
) -> Result<(), String> {
    // Packet -> [type: u8, size:u16] 3 bytes -> Data[size]

    let mut my_stream = stream.try_clone().unwrap();

    loop {
        let my_stream2 = my_stream.try_clone().unwrap();
        let peer_addr = get_peer_addr(&my_stream2)?;

        println!("[{peer_addr}] Blocking on read");

        let mut buffer = [0; 1500];

        match my_stream.read(&mut buffer) {
            Ok(bytes) => {
                // Stream is closed
                if bytes == 0 {
                    println!("[{}] Stream closed", peer_addr);
                    return Ok(());
                }

                let packet_size = u16::from_be_bytes(buffer[1..3].try_into().unwrap());

                let mut packet = match PacketType::from_u8(
                    buffer[0],
                    buffer[3..packet_size as usize].to_vec(),
                ) {
                    Some(packet) => packet,
                    None => return Err("Invalid packet type".to_string()),
                };

                println!(
                    "[{}] Packet received with type: {}, size: {}",
                    peer_addr, buffer[0], packet_size
                );

                let changed = match packet.handle(my_stream2, table) {
                    Ok(changed) => changed,
                    Err(err) => {
                        println!("{}", err);
                        false
                    }
                };

                if changed {
                    {
                        let lock = table.write().unwrap();

                        let flood_pack = FloodPacket::from_link(&lock.closest_link);

                        for l in lock.links.iter() {
                            if l.addr == lock.closest_link.addr {
                                continue;
                            }

                            //println!("Sending flood to {}", l.addr);
                            let mut stream =
                                TcpStream::connect(format!("{}:{}", l.addr.clone(), LISTENER_PORT))
                                    .unwrap();
                            stream
                                .write(flood_pack.to_bytes().unwrap().as_ref())
                                .unwrap();
                        }
                    }
                }

                println!("[{}] Packet handled {}", peer_addr, buffer[0]);
            }
            Err(_) => {
                println!("[{}] Something went wrong with the connection", peer_addr);
                return Err("Error reading from stream".to_string());
            }
        };
    }
}
