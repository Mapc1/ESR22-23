#![allow(non_snake_case)]

use std::{
    net::TcpStream,
    env
};
use lib::{
    http::connection::get_request,
    logging::logger::Logger,
    node::routing::{
        overlay_graph::OverlayGraph,
        routing_table::RoutingTable
    },
};


static INFO: bool = true;
static ERROR: bool = true;
static DBG: bool = true;

fn request_file(bootstrapper_addr: &String) -> Result<String, String> {
    let mut stream = match TcpStream::connect(bootstrapper_addr) {
        Ok(stream) => stream,
        Err(_) => return Err("Error connecting to server. Perhaps it's down?".to_string()),
    };

    let body = match get_request(&mut stream, "Olá ^.^") {
        Ok(body) => body,
        Err(_) => return Err("Error requesting file from bootstrapper".to_string()),
    };

    Ok(body)
}

fn main() -> Result<(),()>{
    let logger = Logger::new(INFO, ERROR, DBG);

    let args: Vec<String> = env::args().collect();
    let bootstrapper_addr = match args.get(1) {
        Some(addr) => addr,
        None => {
            logger.log_error(
                "This program requires the bootstrapper ip address as an argument, but none were given".to_string()
            );
            return Err(());
        }
    };
     
    logger.log_info(
        "Hello! Requesting topology from bootstrap server".to_string()
    );
    let file = match request_file(bootstrapper_addr) {
        Ok(content) => content,
        Err(error) => {
            logger.log_error(error.to_string());
            return Err(());
        }
    };
    logger.log_info(
        "File received successfully. Parsing...".to_string()
    );
    logger.log_dbg(format!("File received: {file:#?}"));

    let o_graph = match OverlayGraph::parse_graph_file(file, "10.0.3.2".to_string()) {
        Ok(graph) => graph,
        Err(msg) => {
            logger.log_error(msg);
            return Err(())
        }
    };
    logger.log_dbg(format!("Overlay graph parsed: {o_graph:#?}"));

    let routing_tab = match RoutingTable::calc_paths(o_graph, "10.0.3.2".to_string()) {
        Ok(table) => table,
        Err(msg) => {
            logger.log_error(msg);
            return Err(())
        }
    };
    logger.log_dbg(format!("Routing table calculated: {routing_tab:#?}"));

    Ok(())
}
