use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use std::thread;
use std::thread::Thread;
use std::time::Duration;

pub static TIMEOUT: Duration = Duration::new(5, 0);
pub static RETRY_TIMES: u32 = 5;

pub fn get_peer_addr(stream: TcpStream) -> String {
    stream.peer_addr().unwrap().ip().to_string()
}

pub fn connect(peer_addr: &String) -> Result<TcpStream, String> {
    let socket_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 10)), 8080);

    for RETRY_TIME in 1..RETRY_TIMES {
        println!("Try number: {RETRY_TIME}");
        match TcpStream::connect_timeout(&socket_addr, TIMEOUT) {
            Ok(stream) => return Ok(stream),
            Err(_) => {
                thread::sleep(TIMEOUT);
                continue;
            }
        };
    }

    Err("Error connecting to server. Perhaps it's down?".to_string())
}
