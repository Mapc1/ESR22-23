#![allow(non_snake_case)]

static INFO: bool = true;
static ERROR: bool = true;
static DBG: bool = true;

use std::{
    net::TcpStream,
    env
};

use lib::{
    http::connection::get_request,
    logging::logger::Logger, 
    node::routing::overlay_graph::OverlayGraph,
};

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
    logger.log_dbg(format!("{file:?}"));

    let o_graph = OverlayGraph::parse_graph_file(file);
    logger.log_dbg(format!("{o_graph:?}"));

    Ok(())
}
