use std::net::TcpStream;

use crate::node::flooding::routing_table::RoutingTable;
use crate::node::packets::packet::Packet;
use crate::node::packets::utils::get_peer_addr;

#[derive(Serialize, Deserialize, Debug)]
pub struct RefusePacket {}

impl RefusePacket {
    pub fn from_bytes_packet_type(_bytes: Vec<u8>) -> RefusePacket {
        todo!()
    }
}

impl Packet for RefusePacket {
    /*
    Returns true if the node becomes inactive
     */
    fn handle(&self, mut stream: TcpStream, table: &mut RoutingTable) -> Result<bool, String> {
        return match table.handle_teardown_packet(get_peer_addr(stream)) {
            Ok(_) => {
                if table.has_active_links() {
                    Ok(false)
                } else {
                    // Sends refuse packet to the peer that sends the stream
                    // TODO: Send refuse packet to the peer that sends the stream
                    Ok(true)
                }
            }
            Err(e) => Err("Packet wasn't receiving the stream...".to_string()),
        };
    }
}
