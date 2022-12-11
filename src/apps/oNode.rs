#![allow(non_snake_case)]

use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use std::{env, net::TcpStream};

use lib::node::flooding::routing_table::RoutingTable;
use lib::node::packets::utils::connect;
use lib::{
    http::connection::get_request, logging::logger::Logger, node::threads::listener::listener,
};

static INFO: bool = true;
static ERROR: bool = true;
static DBG: bool = true;

static TIMEOUT: Duration = Duration::new(5, 0);

static RETRY_TIMES: u32 = 5;

fn request_file(bootstrapper_addr: &String) -> Result<String, String> {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 10)), 8080);

    let mut stream = match connect(&"".to_string()) {
        Ok(stream) => stream,
        Err(e) => return Err(e),
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
            logger
                .log_error(
                    "This program requires the machine's own ip as the second argument".to_string(),
                )
                .expect("Log error");
            return Err(());
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
        Err(_) => return Err(()),
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

    loop {}

    logger
        .log_info("oNode is turning off!".to_string())
        .expect("Log info");

    Ok(())
}
