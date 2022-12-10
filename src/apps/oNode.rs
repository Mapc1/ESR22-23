#![allow(non_snake_case)]

use std::{env, net::TcpStream};

use lib::{
    http::connection::get_request, logging::logger::Logger, node::threads::listener::listener,
};
use lib::node::flooding::routing_table::RoutingTable;

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

fn main() -> Result<(), ()> {
    let logger = Logger::new(INFO, ERROR, DBG);

    let args: Vec<String> = env::args().collect();

    let bootstrapper_addr = match args.get(1) {
        Some(addr) => addr,
        None => {
            logger.log_error(
                "This program requires the bootstrapper ip address as the first argument, but none were given".to_string()
            ).expect("Log error");
            return Err(());
        }
    };
    let own_ip = match args.get(2) {
        Some(addr) => addr.to_owned(),
        None => {
            logger.log_error(
                "This program requires the machine's own ip as the second argument".to_string()
            ).expect("Log error");
            return Err(())
        }
    };

    logger
        .log_info("Hello! Requesting topology from bootstrap server".to_string())
        .expect("Log info");
    let file = match request_file(bootstrapper_addr) {
        Ok(content) => content,
        Err(error) => {
            logger.log_error(error.to_string()).expect("Log error");
            return Err(());
        }
    };
    logger
        .log_info("File received successfully. Parsing...".to_string())
        .expect("Log info");
    logger
        .log_dbg(format!("File received: {file:#?}"))
        .expect("Log debug");

    let mut table = match RoutingTable::from_file(file, own_ip) {
        Ok(table) => table,
        Err(err) => return Err(())
    };

    // Creating the needed threads
    logger
        .log_info("Starting listening for connections".to_string())
        .expect("Log info");

    let logger_copy = logger.clone();
    std::thread::spawn(move || match listener(&mut table) {
        Ok(_) => Ok(()),
        Err(error) => {
            logger_copy.log_error(error).expect("Log error");
            Err(())
        }
    });

    loop {

    }

    logger
        .log_info("oNode is turning off!".to_string())
        .expect("Log info");

    Ok(())
}
