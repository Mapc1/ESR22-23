use std::{
    net::{TcpListener, TcpStream, SocketAddr},
    fs,
    io::Write,
    thread
};

static FILE_PATH: &str = "configs/bootstrapper.conf";

fn handle_client(file: String,mut stream: TcpStream, _addr: SocketAddr) {
    stream.write(file.as_bytes()).unwrap();
}

fn main() {
    let file_content = fs::read_to_string(FILE_PATH)
        .expect("Error reading config file");

    let listener = TcpListener::bind("localhost:8080")
        .expect("Couldn't open socket");

    for con in listener.incoming() {
        let stream = con.unwrap();
        let addr = stream.peer_addr().unwrap();
        let copy = file_content.clone();

        thread::spawn(move || {
            handle_client(copy, stream, addr);
        });
    }
}
