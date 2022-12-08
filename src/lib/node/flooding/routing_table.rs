use crate::node::routing::node::Node;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RoutingTable {
    own_ip: String,
    nodes: HashMap<String, Node>,
    table: HashMap<String, (String, u32)>,
}

impl RoutingTable {}
