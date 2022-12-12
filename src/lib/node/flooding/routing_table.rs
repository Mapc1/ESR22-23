use std::time::SystemTime;

use crate::node::packets::flood_packet::FloodPacket;

use super::link::Link;

const _TIME_MARGIN: f32 = 0.10; // TODO: Should we being using this??

#[derive(Debug, Clone)]
pub struct RoutingTable {
    pub links: Vec<Link>,
    pub closest_link: Link,
    pub num_stream_connections: u32,
    pub clients: Vec<String>,
}

impl RoutingTable {
    pub fn new(links: Vec<Link>) -> Self {
        let mut table = Self {
            closest_link: Link::new_default("dummy"),
            links,
            num_stream_connections: 0,
            clients: Vec::new(),
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
        let mut iterator = self.links.iter_mut();
        let link = loop {
            let l = iterator.next().unwrap();
            if l.addr == peer_addr {
                break l;
            }
        };

        let delay = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
            - packet.timestamp;

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

        prev_closest.addr != self.closest_link.addr
    }

    pub fn has_active_connections(&self) -> bool {
        self.num_stream_connections > 0
    }

    /**
    Assuming that the link is already in the links vector:
    */
    pub fn handle_request_packet(&mut self, peer_addr: String) -> Result<bool, String> {
        let mut found_link = false;

        for l in self.links.iter_mut() {
            if l.addr == peer_addr {
                println!("request from {}", l.addr);
                if l.active {
                    return Err("Link is already receiving the stream...".to_string());
                }
                l.active = true;
                l.has_clients = true;
                found_link = true;

                break; // FIXME: Returns hardcoded boolean for now
            }
        }
        if !found_link {
            println!("Request It's a client {peer_addr}");
            // If we got here then the packet was sent from a client
            self.clients.push(peer_addr);
        }
        self.num_stream_connections += 1;

        Ok(true)
    }

    /**
    Assuming that the link is already in the links vector:

    Returns **Ok** if the node becomes inactive.
    */
    pub fn handle_refuse_packet(&mut self, peer_addr: String) -> Result<(), String> {
        for l in self.links.iter_mut() {
            if l.addr == peer_addr {
                println!("Refuse Packet from {}", l.addr);
                l.active = false;
                self.num_stream_connections -= 1;
                return Ok(());
            }
        }

        // If we got here, the refuse is from a client
        for i in 0..self.clients.len() {
            if self.clients[i] == peer_addr {
                println!("Refuse Packet from client {peer_addr}");
                self.clients.remove(i);
                return Ok(());
            }
        }

        Err("Node wasn't receiving the stream...".to_string())
    }
}
