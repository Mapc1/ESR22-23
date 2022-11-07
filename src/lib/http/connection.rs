use std::{net::TcpStream, io::{Write, Read}, str::from_utf8};

pub fn get_request(stream: &mut TcpStream, msg: &str) -> (String, String) {
    stream.write(msg.as_bytes())
        .expect("Error writing to stream buffer");
    
    let mut buf = [0; 1500];
    stream.read(&mut buf)
        .expect("Error reading response");

    let response = from_utf8(&buf)
        .unwrap();

    let (header, body) = response.split_once("\n\n")
        .unwrap();
    
    (header.to_string(), body.to_string())
}