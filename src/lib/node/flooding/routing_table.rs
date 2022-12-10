use std::time::SystemTime;
use crate::node::packets::flood_packet::FloodPacket;
use super::link::Link;

const TIME_MARGIN: f32 = 0.10;

#[derive(Debug, Clone)]
pub struct RoutingTable {
    pub links: Vec<Link>,
    pub closest_link: Link,
}

impl RoutingTable {
    pub fn new(links: Vec<Link>) -> Self {
        let mut table = Self {
            closest_link: Link::new_default("dummy"),
            links
        };
        table.calc_closest_link();
        table
    }

    pub fn from_file<S: Into<String>>(file: S, own_ip: S) -> Result<Self, String> {
        let mut links: Vec<Link> = Vec::new();
        let own_ip = own_ip.into();

        for line in file.into().lines() {
            let (left,right) = match line.split_once("-") {
                Some(val) => val,
                None => return Err("File has an invalid format".to_string()),
            };
            if left != own_ip {
                continue
            }

            for ip in right.split(",") {
                links.push(Link::new_default(ip));
            }

            break
        }

        Ok(Self::new(links))
    }

    pub fn handle_flood_packet(&mut self, peer_addr: String, packet: &FloodPacket) -> Result<bool, String> {
        println!("{peer_addr}");

        let mut iterator = self.links.iter_mut();
        let link = loop {
            let l = iterator.next().unwrap();
            if l.addr == peer_addr {
                break l
            }
        };

        let delay = match SystemTime::now().duration_since(packet.timestamp) {
            Ok(delay) => delay,
            Err(err) => return Err(err.to_string()),
        };

        link.source = packet.source.to_string();
        link.jumps = packet.jumps;
        link.active = true;
        link.delay = delay;

        Ok(self.calc_closest_link())
    }

    fn calc_closest_link(&mut self) -> bool {
        let prev_closest = self.closest_link.clone();
        let mut closest = &self.closest_link;
        for l in self.links.iter() {
            if l.jumps < closest.jumps {
                closest = l;
            }
        }

        self.closest_link = closest.clone();

        prev_closest.source != self.closest_link.source
    }
}
