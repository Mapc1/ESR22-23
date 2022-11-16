use std::collections::HashMap;
use std::net::Ipv4Addr;

use regex::Regex;

use super::node::Node;

#[derive(Debug)]
pub struct OverlayGraph {
    nodes: HashMap<String,Node>
}

impl Default for OverlayGraph {
    fn default() -> Self {
        Self { nodes: HashMap::new() }
    }
}

impl OverlayGraph {
    pub fn parse_graph_file(file: String, own_ip: String) -> Result<Self, String> {
        let reg = Regex::new(r"^(\d{1,3}).(\d{1,3}).(\d{1,3}).(\d{1,3})$").unwrap();
        let mut o_graph = OverlayGraph::default();

        for line in file.trim_matches('\0').lines() {
            let (left, right) = match line.split_once("-") {
                Some(val) => val,
                None => return Err("File is an invalid format".to_string())
            };

            let captures = match reg.captures(left) {
                Some(caps) => caps,
                None => return Err(format!("Ip address '{left}' is in an invalid format"))
            };

            let mut links: Vec<String> = Vec::new();
            for ip in right.split(",") {
                if own_ip != ip {
                    links.push(ip.to_string());
                }
            }

            let node = Node::new(
                false,
                Ipv4Addr::new(
                    (&captures[1]).parse().unwrap(),
                    (&captures[2]).parse().unwrap(),
                    (&captures[3]).parse().unwrap(),
                    (&captures[4]).parse().unwrap()
                ),
                links
            );
            o_graph.add_node(node);
        }

        Ok(o_graph)
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.get_ip().to_string(),node);
    }

    pub fn get_nodes(&self) -> HashMap<String, Node> {
        self.nodes.clone()
    }
}
