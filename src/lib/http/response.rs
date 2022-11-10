use std::{
    net::TcpStream,
    io::{self,Write},
};

use super::status::Status;

pub fn respond(stream: &mut TcpStream, msg: String, status: Status) -> io::Result<usize> {
    let header = status.get_status_header();

    stream.write(
        format!("{header}\n\n{msg}").as_bytes()
    )
}