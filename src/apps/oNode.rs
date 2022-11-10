use std::{
    net::TcpStream,
    env
};

use lib::http::connection::get_request;

fn request_file(bootstrapper_addr: &String) -> String {
    let mut stream = TcpStream::connect(bootstrapper_addr)
        .expect("Error connecting to server. Perhaps it's down?");

    let (_, body) = get_request(&mut stream, "Olá ^.^");

    body
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let bootstrapper_addr = args.get(1)
        .expect("This program requires the bootstrapper ip address as an argument, but none were given");
        
    let file = request_file(bootstrapper_addr);

    println!("{}", file);
}
