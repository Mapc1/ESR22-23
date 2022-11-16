use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct Node {
    active: bool,
    ip: Ipv4Addr,
    links: Vec<String>
}

impl Node {
    pub fn new(active: bool, ip: Ipv4Addr, links: Vec<String>) -> Node {
        Node {
            active,
            ip,
            links
        }
    }

    pub fn get_ip(&self) -> Ipv4Addr {
        self.ip.clone()
    }

    pub fn get_links(&self) -> Vec<String> {
        self.links.clone()
    }

    pub fn get_active(&self) -> bool {
        self.active.clone()
    }
}