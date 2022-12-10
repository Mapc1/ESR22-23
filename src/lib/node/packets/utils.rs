use std::net::TcpStream;

pub fn get_peer_addr(stream: TcpStream) -> String {
    stream.peer_addr().unwrap().ip().to_string()
}
