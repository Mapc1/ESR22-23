#![allow(non_snake_case)]

use std::env;
use std::sync::{Arc, RwLock};

use lib::node::flooding::routing_table::RoutingTable;
use lib::node::packets::utils::connect;
use lib::node::threads::listener::udp_listener;
use lib::{
    http::connection::get_request, logging::logger::Logger, node::threads::listener::listener,
};

static INFO: bool = true;
static ERROR: bool = true;
static DBG: bool = true;

static BOOTSTRAPPER_PORT: u16 = 8080;

static DEFAULT_BOOTSTRAPPER_ADDR: &str = "10.0.0.10";

fn request_file(bootstrapper_addr: &String) -> Result<String, String> {
    let mut stream = match connect(bootstrapper_addr, BOOTSTRAPPER_PORT) {
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

    let own_ip = match args.get(1) {
        Some(addr) => addr.to_owned(),
        None => {
            logger
                .log_error(
                    "This program requires the machine's own ip as the first argument".to_string(),
                )
                .expect("Log error");

            return Err(());
        }
    };

    let bootstrapper_addr = match args.get(2) {
        Some(addr) => addr,
        None => {
            logger.log_error(
                "No bootstrapper ip address as the second argument, searching for the default bootstrap address...".to_string()
            ).expect("Log error");

            DEFAULT_BOOTSTRAPPER_ADDR
        }
    };

    logger
        .log_info("Hello! Requesting topology from bootstrap server".to_string())
        .expect("Log info");
    let file = match request_file(&bootstrapper_addr.to_string()) {
        Ok(content) => content,
        Err(error) => {
            logger.log_error(error.to_string()).expect("Log error");
            return Err(());
        }
    };
    logger
        .log_info("File received successfully. Parsing...".to_string())
        .expect("Log info");
    /*
    logger
        .log_dbg(format!("File received: {file:#?}"))
        .expect("Log debug");
     */

    let table = match RoutingTable::from_file(file, own_ip) {
        Ok(table) => table,
        Err(_) => {
            logger
                .log_error("Error initializing routing table".to_string())
                .expect("Log error");
            return Err(());
        }
    };

    let mut shared_mem = Arc::new(RwLock::new(table));

    // Creating the needed threads
    logger
        .log_info("Starting listening for connections".to_string())
        .expect("Log info");

    let logger_copy = logger.clone();
    let mut shared_mem_cpy = shared_mem.clone();
    std::thread::spawn(move || match listener(&mut shared_mem_cpy) {
        Ok(_) => Ok(()),
        Err(error) => {
            logger_copy.log_error(error).expect("Log error");
            Err(())
        }
    });
    let logger_copy = logger.clone();
    std::thread::spawn(move || match udp_listener(&mut shared_mem) {
        Ok(_) => Ok(()),
        Err(error) => {
            logger_copy.log_error(error).expect("Log error");
            Err(())
        }
    });

    loop {}

    //logger
    //    .log_info("oNode is turning off!".to_string())
    //    .expect("Log info");

    //Ok(())
}
