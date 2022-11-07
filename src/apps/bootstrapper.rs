use std::{
    net::{TcpListener, TcpStream, SocketAddr},
    fs,
    io::{Write, Read},
    thread
};

use lib::http::status::Status;

static FILE_PATH: &str = "configs/bootstrapper.conf";

fn handle_client(file: String,mut stream: TcpStream, _addr: SocketAddr) {
    let mut buf = [0; 1500];

    let header: String;
    match stream.read(&mut buf) {
        Ok(_) => header = Status::OK.get_status_header(),
        Err(_) => header = Status::ERROR.get_status_header()
    }

    stream.write(format!("{}\n\n{}", header, file).as_bytes()).unwrap();
}

fn main() {
    println!("Hello!\nReading config file...");
    let file_content = fs::read_to_string(FILE_PATH)
        .expect("Error reading config file");

    let listener = TcpListener::bind("0.0.0.0:8080")
        .expect("Couldn't open socket");

    println!("Bootstrap server ready!\nListening on port {}...", listener.local_addr().unwrap().port());
    for con in listener.incoming() {
        let stream = con.unwrap();
        let addr = stream.peer_addr().unwrap();
        let copy = file_content.clone();

        println!("Accepted connection from {}", stream.peer_addr().unwrap());
        thread::spawn(move || {
            handle_client(copy, stream, addr);
        });
    }
}
