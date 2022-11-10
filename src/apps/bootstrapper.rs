use std::{
    net::{TcpListener, TcpStream, SocketAddr},
    fs,
    io::{Write, Read},
    thread,
};

use lib::{
    http::status::Status,
    logging::logger::Logger,
};

static FILE_PATH: &str = "configs/bootstrapper.conf";
static LISTENNING_ADDRESS: &str = "0.0.0.0:8080";

fn handle_client(file: String,mut stream: TcpStream, _addr: SocketAddr) {
    let mut buf = [0; 1500];

    let header = match stream.read(&mut buf) {
        Ok(_) => Status::OK.get_status_header(),
        Err(_) => Status::ERROR.get_status_header()
    };

    stream.write(format!("{}\n\n{}", header, file).as_bytes()).unwrap();
}

fn main() -> Result<(),()> {
    let logger = Logger::new(true, true, true);

    logger.log_info("Hello! Reading config file...".to_string());
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(content) => content,
        Err(_) => {
            logger.log_error("Error reading file contents!".to_string());
            return Err(());
        }
    };

    let listener = match TcpListener::bind(LISTENNING_ADDRESS) {
        Ok(socket) => socket,
        Err(_) => {
            logger.log_error(format!("Error openning socket at {}", LISTENNING_ADDRESS));
            return Err(());
        }
    };

    logger.log_info(format!("Bootstrap server ready! Listening on port {}...", listener.local_addr().unwrap().port()));
    for con in listener.incoming() {
        let stream = con.unwrap();
        let addr = stream.peer_addr().unwrap();
        let copy = file_content.clone();

        logger.log_dbg(format!("Accepted connection from {}", stream.peer_addr().unwrap()));
        thread::spawn(move || {
            handle_client(copy, stream, addr);
        });
    }

    Ok(())
}
