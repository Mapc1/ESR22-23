use std::net::TcpStream;
use std::time::SystemTime;
use rmp_serde::{Deserializer, Serializer};

use serde::{Deserialize, Serialize};

use crate::node::flooding::link::Link;
use crate::node::packets::packet::Packet;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct FloodPacket {
    source: String,
    cost: u32,
    timestamp: SystemTime,
}

impl FloodPacket {
    pub fn new(source: String, cost: u32, timestamp: SystemTime) -> Self {
        Self {
            source,
            cost,
            timestamp,
        }
    }

    pub fn from_bytes_packet_type(bytes: Vec<u8>) -> FloodPacket {
        let mut des = rmp_serde::Deserializer::new(&bytes[..]);

        Deserialize::deserialize(&mut des).expect("ola")
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, String> {
        let pack_type: u8 = 0; // FIXME
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

impl Packet for FloodPacket {
    // FIXME
    fn handle(&self, mut stream: TcpStream, links: &Vec<Link>) {
        let mut i = 0;
        /*let link = loop {
            let l = links.get(i).unwrap();
            if l.source == self.source {
                break l;
            }
            i += 1;
        };
*/
        println!("{:#?}", self);
    }
}
