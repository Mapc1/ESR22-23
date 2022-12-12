use std::net::TcpStream;
use std::thread;
use std::time::Duration;

pub static TIMEOUT: Duration = Duration::new(5, 0);
pub static RETRY_TIMES: u32 = 5;

pub fn get_peer_addr(stream: &TcpStream) -> String {
    stream.peer_addr().unwrap().ip().to_string()
}

pub fn connect(peer_addr: &String, port: u16) -> Result<TcpStream, String> {
    let socket_addr = match format!("{peer_addr}:{port}").parse() {
        Ok(addr) => addr,
        Err(_) => {
            return Err(format!("Cannot create socket address {peer_addr}: {port}").to_string())
        }
    };

    for _ in 1..RETRY_TIMES {
        match TcpStream::connect_timeout(&socket_addr, TIMEOUT) {
            Ok(stream) => return Ok(stream),
            Err(_) => {
                thread::sleep(TIMEOUT);
                println!("Retrying connection to {peer_addr}");
                continue;
            }
        };
    }

    Err("Error connecting to peer. Perhaps it's down?".to_string())
}
