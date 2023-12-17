use std::net::UdpSocket;

use crate::dns::message::Message;
mod dns;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from the program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                let received_data = &buf[0..size];
                println!("Received {} bytes from {}", size, source);
                println!("Received data: {received_data:#04X?}");
                let message = Message::gen_response(&buf, size);
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
