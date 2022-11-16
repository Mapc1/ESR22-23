use super::{
    node::Node,
    overlay_graph::OverlayGraph,
    algorithms::dijkstra::DijkstraTable
};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct RoutingTable {
    own_ip: String,
    nodes: HashMap<String, Node>,
    table: HashMap<String, (String, u32)>,
}

impl RoutingTable {
    pub fn calc_paths(o_graph: OverlayGraph, ip: String) -> Result<Self, String> {
        let nodes = o_graph.get_nodes();
        let table = DijkstraTable::calc_routes(&nodes, ip.clone())?;

        Ok(RoutingTable {
            own_ip: ip,
            nodes,
            table
        })
    }
}