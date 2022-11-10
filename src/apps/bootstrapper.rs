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


fn handle_client(file: String, mut stream: TcpStream, logger: Arc<Mutex<Logger>>) {
    let mut buf = [0; 1500];

    if let Err(error) = stream.read(&mut buf) {
        logger.lock().unwrap().log_error(
            error.to_string()
        );
        return;
    };

    if let Err(error) = respond(&mut stream, file, Status::OK) {
        logger.lock().unwrap().log_error(error.to_string())
    };
}

fn main() -> Result<(),()> {
    let logger = Arc::new(
        Mutex::new(
            Logger::new(true, true, true)
        )
    );

    logger.lock().unwrap().log_info(
        "Hello! Reading config file...".to_string()
    );
    let file_content = match fs::read_to_string(FILE_PATH) {
        Ok(content) => content,
        Err(_) => {
            logger.lock().unwrap().log_error(
                "Error reading file contents!".to_string()
            );
            return Err(());
        }
    };

    let listener = match TcpListener::bind(LISTENNING_ADDRESS) {
        Ok(socket) => socket,
        Err(_) => {
            logger.lock().unwrap().log_error(
                format!("Error openning socket at {LISTENNING_ADDRESS}")
            );
            return Err(());
        }
    };

    let local_port = listener.local_addr().unwrap().port();
    logger.lock().unwrap().log_info(
        format!("Bootstrap server ready! Listening on port {local_port}...")
    );
    
    for con in listener.incoming() {
        let stream = con.unwrap();
        let peer_addr = stream.peer_addr().unwrap();
        let copy = file_content.clone();
        let logger_cpy = logger.clone();

        logger.lock().unwrap().log_dbg(
            format!("Accepted connection from {peer_addr}")
        );
        thread::spawn(move || {
            handle_client(copy, stream, logger_cpy);
        });
    }

    Ok(())
}
