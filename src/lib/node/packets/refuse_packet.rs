use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

use serde::{Deserialize, Serialize};

use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::packet::Packet;
use crate::node::packets::utils::get_peer_addr;
use crate::node::threads::listener::LISTENER_PORT;

use super::utils::connect;

#[derive(Serialize, Deserialize, Debug)]
pub struct RefusePacket {}

impl RefusePacket {
    pub fn new() -> Self {
        Self { }
    }

    pub fn from_bytes_packet_type(bytes: Vec<u8>) -> RefusePacket {
        Self::new()
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        Ok(vec![self.get_type(), 3])
    }
}

impl Packet for RefusePacket {
    fn get_type(&self) -> u8 {
        3
    }

    /*
    Returns true if the node becomes inactive
     */
    fn handle(&self, stream: TcpStream, table: &mut Arc<RwLock<RoutingTable>>) -> Result<bool, String> {
        let mut table_lock = table.write().unwrap();
        return match table_lock.handle_teardown_packet(get_peer_addr(&stream)) {
            Ok(_) => {
                if table_lock.has_active_connections() {
                    Ok(false)
                } else {
                    // Sends refuse packet to the peer that sends the stream that is the closest link
                    // TODO: Send refuse packet to the peer that sends the stream
                    let mut back_stream = connect(&table_lock.closest_link.addr, LISTENER_PORT).unwrap();
                    back_stream.write(&self.to_bytes().unwrap()).unwrap();
                    Ok(true)
                }
            }
            Err(e) => Err(e),
        };
    }
}
