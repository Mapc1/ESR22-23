use std::{
    io::{Read, Write},
    net::TcpStream,
    str::from_utf8,
};

pub fn get_request(stream: &mut TcpStream, msg: &str) -> Result<String, ()> {
    match stream.write(msg.as_bytes()) {
        Ok(_) => {
            let mut buf = [0; 1500];
            stream.read(&mut buf).expect("Error reading response");

            let response = from_utf8(&buf).unwrap();

            let (_, body) = response.split_once("\n\n").unwrap();

            Ok(body.to_string())
        }
        Err(_) => Err(()),
    }

}