use std::env;
use std::{
    fs,
    io::Read,
    net::{TcpListener, TcpStream},
    thread,
};

use lib::types::networking::Addr;
use lib::{
    http::{response::respond, status::Status},
    logging::logger::Logger,
};

static DEFAULT_FILE_PATH: &str = "configs/bootstrapper.conf";
static LISTENING_ADDRESS: &Addr = "0.0.0.0:8080";

static INFO: bool = true;
static ERROR: bool = true;
static DEBUG: bool = true;

fn handle_client(file: String, mut stream: TcpStream, logger: Logger) {
    let mut buf = [0; 1500];

    if let Err(error) = stream.read(&mut buf) {
        logger.log_error(error.to_string()).expect("Log error");
        return;
    };

    if let Err(error) = respond(&mut stream, file, Status::OK) {
        logger.log_error(error.to_string()).expect("Log error")
    };
}

fn main() -> Result<(), ()> {
    let logger = Logger::new(INFO, ERROR, DEBUG);

    let args: Vec<String> = env::args().collect();

    let bootstrapper_file_path = match args.get(1) {
        Some(path) => path,
        None => {
            logger
                .log_info(
                    "No bootstrapper file path as the first argument, searching for default file path..."
                        .to_string(),
                )
                .expect("Log info");
            DEFAULT_FILE_PATH
        }
    };

    logger
        .log_info("Hello! Reading config file...".to_string())
        .expect("Log info");
    let file_content = match fs::read_to_string(bootstrapper_file_path) {
        Ok(content) => content,
        Err(_) => {
            logger
                .log_error(
                    "Error reading file contents! Perhaps the file path is wrong?".to_string(),
                )
                .expect("Log error");
            return Err(());
        }
    };

    let listener = match TcpListener::bind(LISTENING_ADDRESS) {
        Ok(socket) => socket,
        Err(_) => {
            logger
                .log_error(format!("Error opening socket at {LISTENING_ADDRESS}"))
                .expect("Log error");
            return Err(());
        }
    };

    let local_port = match listener.local_addr() {
        Ok(addr) => addr.port(),
        Err(err) => {
            logger.log_error(err.to_string()).expect("Log error");
            return Err(());
        }
    };
    logger
        .log_info(format!(
            "Bootstrap server ready! Listening on port {local_port}..."
        ))
        .expect("Log info");

    for con in listener.incoming() {
        let stream = match con {
            Ok(stream) => stream,
            Err(err) => {
                logger.log_error(err.to_string()).expect("Log error");
                continue;
            }
        };
        let peer_addr = match stream.peer_addr() {
            Ok(addr) => addr,
            Err(err) => {
                logger.log_error(err.to_string()).expect("Log error");
                continue;
            }
        };
        let copy = file_content.clone();
        let logger_cpy = logger.clone();

        logger
            .log_dbg(format!("Accepted connection from {peer_addr}"))
            .expect("Log debug");
        thread::spawn(move || {
            handle_client(copy, stream, logger_cpy);
        });
    }

    Ok(())
}
