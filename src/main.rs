use anyhow::Result;
use codecrafters_dns_server::dns::{header, question, answer};
use std::net::UdpSocket;

fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {size} bytes from {source}");
                let mut offset = 0;

                let flags = header::Flags::default().with_qr_indicator(1).into();

                let header = header::Header {
                    packet_identifier: 1234,
                    qd_count: 1,
                    an_count: 1,
                    flags,
                    ..header::Header::default()
                };

                let header_bytes = header.to_bytes();
                offset += header_bytes.len();
                buf[0..offset].copy_from_slice(&header_bytes);

                let question = question::Question::new("codecrafters.io".to_string());
                let question_bytes = question.to_bytes();
                buf[offset..offset + question_bytes.len()].copy_from_slice(&question_bytes);

                offset += question_bytes.len();

                let answer = answer::Answer::new("codecrafters.io".to_string());
                let answer_bytes = answer.to_bytes();
                buf[offset..offset + answer_bytes.len()].copy_from_slice(&answer_bytes);

                offset += answer_bytes.len();


                udp_socket
                    .send_to(&buf, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {e}");
                break;
            }
        }
    }

    Ok(())
}
