use std::{
    net::{TcpStream}
};

use lib::http::connection::get_request;

static BOOTSTRAPPER_ADDRESS: &str = "localhost:8080"; 

fn request_file() -> String {
    let mut stream = TcpStream::connect(BOOTSTRAPPER_ADDRESS)
        .expect("Error connecting to server. Perhaps it's down?");

    let (_, body) = get_request(&mut stream, "Olá ^.^");

    body
}

fn main() {
    let file = request_file();

    println!("{}", file);
}