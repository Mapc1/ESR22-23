use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8, fmt::Error,
};

pub fn get_request(stream: &mut TcpStream, msg: &str) -> Result<String, Error> {
    match stream.write(msg.as_bytes()) {
        Ok(_) => {
            let mut buf = [0; 1500];
            stream.read(&mut buf).expect("Error reading response");

            let response = from_utf8(&buf).unwrap();

            let (header, body) = response.split_once("\n\n").unwrap();

            (header.to_string(), body.to_string())
        }
        Err(_) => Error,
    }

}