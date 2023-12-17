use std::net::{Ipv4Addr, UdpSocket};

use crate::dns::{answer::Answer, header::Header, message::Message, question::Question};
mod dns;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from the program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let _received_data = String::from_utf8_lossy(&buf[0..size]);
                println!("Received {} bytes from {}", size, source);
                let header = Header::new(1234, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0);
                let questions = vec![Question::new("codecrafters.io".to_string(), 1, 1)];
                let answers = vec![Answer::new(
                    "codecrafters.io".to_string(),
                    1,
                    1,
                    60,
                    4,
                    Ipv4Addr::new(8, 8, 8, 8).octets().to_vec(),
                )];
                let message = Message::new(header, questions, answers);
                let response = message.as_bytes();
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
