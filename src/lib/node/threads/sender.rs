use std::net::TcpStream;

const SENDER_IP: &str = "0.0.0.0";
const SENDER_PORT: u16 = 4000;

pub fn sender() -> () {
    let _listener = match TcpStream::connect(format!("{SENDER_IP}:{SENDER_PORT}")) {
        Ok(listener) => listener,
        Err(_) => {
            print!("Error binding listener");
            return;
        }
    };
}
