use std::net::Ipv4Addr;

#[derive(Debug)]
pub struct Node {
    active: bool,
    pub ip: Ipv4Addr,
    pub links: Vec<String>
}

impl Node {
    pub fn new(active: bool, ip: Ipv4Addr, links: Vec<String>) -> Node {
        Node {
            active,
            ip,
            links
        }
    }
}