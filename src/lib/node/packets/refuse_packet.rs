use std::net::TcpStream;

use serde::{Deserialize, Serialize};

use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::packet::Packet;
use crate::node::packets::utils::get_peer_addr;

#[derive(Serialize, Deserialize, Debug)]
pub struct RefusePacket {}

impl RefusePacket {
    pub fn from_bytes_packet_type(bytes: Vec<u8>) -> RefusePacket {
        let mut des = rmp_serde::Deserializer::new(&bytes[..]);

        Deserialize::deserialize(&mut des).expect("Something went wrong deserializing")
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let pack_type: u8 = 3; // FIXME
        let mut data = match rmp_serde::to_vec(self) {
            Ok(vec) => vec,
            Err(err) => return Err(err.to_string()),
        };
        let mut size = (data.len() as u16 + 3).to_be_bytes().to_vec();

        let mut pack_data = vec![pack_type];
        pack_data.append(&mut size);
        pack_data.append(&mut data);

        Ok(pack_data)
    }
}

impl Packet for RefusePacket {
    fn get_type(&self) -> u8 {
        3
    }

    /*
    Returns true if the node becomes inactive
     */
    fn handle(&self, stream: TcpStream, table: &mut RoutingTable) -> Result<bool, String> {
        return match table.handle_teardown_packet(get_peer_addr(stream)) {
            Ok(_) => {
                if table.has_active_links() {
                    Ok(false)
                } else {
                    // Sends refuse packet to the peer that sends the stream that is the closest link
                    // TODO: Send refuse packet to the peer that sends the stream
                    Ok(true)
                }
            }
            Err(e) => Err(e),
        };
    }
}
