use std::{
    net::{TcpListener, TcpStream},
    fs,
    io::Read,
    thread, sync::{Mutex, Arc},
};

use lib::{
    http::{
        status::Status,
        response::respond
    },
    logging::logger::Logger,
};


static FILE_PATH: &str = "configs/bootstrapper.conf";
static LISTENNING_ADDRESS: &str = "0.0.0.0:8080";

static INFO: bool = true;
static ERROR: bool = true;
static DEBUG: bool = true;

fn handle_client(file: String, mut stream: TcpStream, logger: Logger) {
    let mut buf = [0; 1500];

    if let Err(error) = stream.read(&mut buf) {
        logger.log_error(error.to_string())
            .expect("Log error");
        return;
    };

    if let Err(error) = respond(&mut stream, file, Status::OK) {
        logger.log_error(error.to_string())
            .expect("Log error")
    };
}

fn main() -> Result<(),()> {
    let logger = Logger::new(INFO, ERROR, DEBUG);

    logger.log_info("Hello! Reading config file...".to_string())
        .expect("Log info");
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(content) => content,
        Err(_) => {
            logger.log_error("Error reading file contents!".to_string())
                .expect("Log error");
            return Err(());
        }
    };

    let listener = match TcpListener::bind(LISTENNING_ADDRESS) {
        Ok(socket) => socket,
        Err(_) => {
            logger.log_error(format!("Error opening socket at {LISTENNING_ADDRESS}"))
                .expect("Log error");
            return Err(());
        }
    };

    let local_port = listener.local_addr().unwrap().port();
    logger.log_info(format!("Bootstrap server ready! Listening on port {local_port}..."))
        .expect("Log info");
    
    for con in listener.incoming() {
        let stream = con.unwrap();
        let peer_addr = stream.peer_addr().unwrap();
        let copy = file_content.clone();
        let logger_cpy = logger.clone();

        logger.log_dbg(format!("Accepted connection from {peer_addr}"))
            .expect("Log debug");
        thread::spawn(move || {
            handle_client(copy, stream, logger_cpy);
        });
    }

    Ok(())
}
