use std::{net::{TcpStream, UdpSocket}, str::from_utf8, sync::{Arc, Mutex, RwLock}};

use serde::{Serialize, Deserialize};
use crate::node::{flooding::routing_table::RoutingTable, threads::listener::LISTENER_PORT};

use super::packet::{Packet, PacketType};

#[derive(Serialize, Deserialize, Debug)]
pub struct StreamPacket {
    pub data: Vec<u8>
}

impl StreamPacket {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    } 

    pub fn from_bytes_packet_type(bytes: Vec<u8>) -> StreamPacket {
        Self { data: bytes }
    }

    pub fn to_bytes(&mut self) -> Result<Vec<u8>, String> {
        let pack_type: u8 = 3;
        let mut size = (self.data.len() as u16 + 3).to_be_bytes().to_vec();

        let mut pack_data = vec![pack_type];
        pack_data.append(&mut size);
        pack_data.append(&mut self.data);

        Ok(pack_data)
    }

    pub fn handle(&mut self, socket: &UdpSocket, peer_addr: String, table: &mut Arc<RwLock<RoutingTable>>) -> Result<bool, String> {
        println!("{}", from_utf8(&self.data).unwrap());
        let lock = table.read().unwrap();

        for link in lock.links.iter() {
            if !link.has_clients || link.addr == peer_addr {
                continue;
            }

            println!("Sending stream to {}",link.addr);
            let pack_bytes = self.to_bytes()?;
            if let Err(err) = socket.send_to(&pack_bytes, format!("{}:{}", link.addr, LISTENER_PORT)) {
                println!("{}", err.to_string());
            }
        }

        Ok(true)
    }
}