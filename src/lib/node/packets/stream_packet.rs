use std::{
    net::UdpSocket,
    str::from_utf8,
    sync::{Arc, RwLock}
};

use serde::{Serialize, Deserialize};
use crate::node::{flooding::routing_table::RoutingTable, threads::listener::LISTENER_PORT};

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
        let size = (self.data.len() as u16 + 3).to_be_bytes().to_vec();

        let mut pack_data = size;
        pack_data.append(&mut self.data);

        Ok(pack_data)
    }

    pub fn handle(&mut self, socket: &UdpSocket, peer_addr: String, table: &mut Arc<RwLock<RoutingTable>>) -> Result<bool, String> {
        let pack_bytes = self.to_bytes()?;
        let read_lock = table.read().unwrap();
        
        //let mut write_lock = read_lock.table.write().unwrap();
        for link in read_lock.links.iter() {
            if !link.has_clients || link.addr == peer_addr {
                continue;
            }
        

            println!("Sending {} bytes to stream {}", self.data.len(), link.addr);
            if let Err(err) = socket.send_to(&pack_bytes, format!("{}:{}", link.addr, LISTENER_PORT)) {
                println!("{}", err.to_string());
            }
        }

        for client in read_lock.clients.iter() {
            socket.send_to(&pack_bytes, format!("{}:{}", client, LISTENER_PORT)).unwrap();
        }

        Ok(true)
    }
}