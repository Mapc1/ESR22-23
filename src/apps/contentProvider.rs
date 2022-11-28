use std::net::UdpSocket;

const LISTENING_ADDRESS: &str = "0.0.0.0:9000";
const PACKET_SIZE: usize = 1200;

const CONTENT_PATH: &str = "";

fn main() -> std::io::Result<()> {
    {
        let mut buf = [0; PACKET_SIZE];
        let socket = UdpSocket::bind(LISTENING_ADDRESS)?;

        // Client establishes connection
        let (size, src) = socket.recv_from(&mut buf)?;
        let buf = &mut buf[..size];

        // Server sends the content as numbered packets
        socket.send_to(buf, &src)?;
    }
    Ok(())
}
