use std::collections::HashMap;
use crate::node::routing::node::Node;

#[derive(Debug)]
pub struct DijkstraTable {
    own_ip: String,
    table: HashMap<String, (String, u32)>,
    remaining_nodes: HashMap<String, Node>
}

impl DijkstraTable {
    fn new(nodes: &HashMap<String,Node>, ip: String) -> Self {
        let mut tab: HashMap<String, (String, u32)> = HashMap::new();

        for (addr,_) in nodes {
            tab.insert(
                addr.clone(),
                ("Empty".to_string(), u32::MAX)
            );
        }
        tab.remove(&ip);

        DijkstraTable {
            own_ip: ip,
            table: tab,
            remaining_nodes: nodes.clone()
        }
    }

    fn get_closest_node(&self) -> Result<(String, u32), String> {
        let mut shortest_path = u32::MAX;
        let mut closest_node = "".to_string();

        for (addr, _) in self.remaining_nodes.iter() {
            let (_, cur_cost) = match self.table.get(addr) {
                Some(cell) => cell.clone(),
                None => return Err(format!(
                    "Node {} does not have a cell in the dijkstra table",
                    addr
                ))
            };

            if cur_cost < shortest_path {
                shortest_path = cur_cost;
                closest_node = addr.clone();
            }
        }
        Ok((closest_node, shortest_path))
    }

    fn calc_next_jump(&self, ip: &String) -> Result<String, String> {
        let mut cur_node = ip.clone();
        let mut closest_link = match self.table.get(&cur_node) {
            Some((addr,_)) => addr.clone(),
            None => return Err(format!(
                "{} is not a node in the overlay graph",
                ip
            ))
        };

        loop {
            if closest_link == self.own_ip {
                return Ok(cur_node)
            }

            cur_node = closest_link.to_string();
            closest_link = match self.table.get(&cur_node) {
                Some((addr,_)) => addr.clone(),
                None => return Err(format!(
                    "{} is not a node in the overlay graph",
                    ip
                ))
            };
        }
    }

    pub fn calc_routes(nodes: &HashMap<String,Node>,  ip: String) -> Result<HashMap<String, (String, u32)>, String> {
        let mut dijkstra_tab = DijkstraTable::new(&nodes, ip.clone());

        let mut cost = 1;
        let mut cur_addr = ip;
        loop {
            let cur_node = match dijkstra_tab.remaining_nodes.get(&cur_addr) {
                Some(node) => node.clone(),
                None => return Err("Dijkstra table has no remaining nodes".to_string())
            };

            for link in cur_node.get_links() {
                let (_, prev_cost) = match dijkstra_tab.table.get(&link) {
                    Some(cell) => cell,
                    None => return Err(format!(
                        "Node '{}' references '{}' as a link but the latter is not a node",
                        cur_node.get_ip().to_string(),
                        link
                    ))
                };
                if cost < *prev_cost {
                    dijkstra_tab.table.insert(link, (cur_addr.to_string(), cost));
                }
            }
            dijkstra_tab.remaining_nodes.remove(&cur_addr);
            if dijkstra_tab.remaining_nodes.is_empty()
            { break }

            let (closest_node, shortest_path) = dijkstra_tab.get_closest_node()?;
            cur_addr = closest_node;
            cost = shortest_path + 1;
        }

        let mut table: HashMap<String, (String, u32)> = HashMap::new();
        for (addr, (link, cost)) in dijkstra_tab.table.iter() {
            let next_jump = dijkstra_tab.calc_next_jump(addr)?;

            table.insert(addr.clone(), (next_jump, *cost));
        }
        Ok(table)
    }
}
