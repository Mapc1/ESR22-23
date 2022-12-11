use std::time::SystemTime;

use crate::node::packets::flood_packet::FloodPacket;

use super::link::Link;

const TIME_MARGIN: f32 = 0.10;

#[derive(Debug, Clone)]
pub struct RoutingTable {
    pub links: Vec<Link>,
    pub closest_link: Link,
    pub num_active_clients: u32,
}

impl RoutingTable {
    pub fn new(links: Vec<Link>) -> Self {
        let mut table = Self {
            closest_link: Link::new_default("dummy"),
            links,
            num_active_clients: 0,
        };
        table.calc_closest_link();
        table
    }

    pub fn from_file<S: Into<String>>(file: S, own_ip: S) -> Result<Self, String> {
        let mut links: Vec<Link> = Vec::new();
        let own_ip = own_ip.into();

        for line in file.into().lines() {
            let (left, right) = match line.split_once("-") {
                Some(val) => val,
                None => return Err("File has an invalid format".to_string()),
            };
            if left != own_ip {
                continue;
            }

            for ip in right.split(",") {
                links.push(Link::new_default(ip));
            }

            break;
        }

        Ok(Self::new(links))
    }

    pub fn handle_flood_packet(
        &mut self,
        peer_addr: String,
        packet: &FloodPacket,
    ) -> Result<bool, String> {
        println!("{peer_addr}");

        let mut iterator = self.links.iter_mut();
        let link = loop {
            let l = iterator.next().unwrap();
            if l.addr == peer_addr {
                break l;
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

    pub fn has_active_links(&self) -> bool {
        self.num_active_clients > 0
    }

    fn get_active_links(&self) -> Vec<Link> {
        let mut active_links: Vec<Link> = Vec::new();
        for l in self.links.iter() {
            if l.active {
                active_links.push(l.clone());
            }
        }
        active_links
    }

    /**
    Assuming that the link is already in the links vector:
    */
    pub fn handle_request_packet(&mut self, peer_addr: String) -> Result<bool, String> {
        for l in self.links.iter_mut() {
            if l.addr == peer_addr {
                if l.active {
                    return Err("Packet is already receiving the stream...".to_string());
                }
                l.active = true;

                return Ok(true); // FIXME: Returns hardcoded boolean for now
            }
        }
        Err("Link not found".to_string())
    }

    /**
    Assuming that the link is already in the links vector:

    Returns **Ok** if the node becomes inactive.
    */
    pub fn handle_teardown_packet(&mut self, peer_addr: String) -> Result<(), String> {
        let mut found = false;

        self.links.iter_mut().for_each(|l| {
            if l.addr == peer_addr {
                l.active = false;
                self.num_active_clients -= 1;
                found = true;
            }
        });

        if found {
            Ok(())
        } else {
            Err("Node wasn't receiving the stream...".to_string())
        }
    }
}
