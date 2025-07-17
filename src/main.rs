use anyhow::Result;
use codecrafters_dns_server::dns::{answer, header, question};
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

                //parses header and flags, returning their structs which can be modified for the
                //response
                let mut parsed_header = header::parse_header(&buf);
                let mut parsed_flags = header::parse_flags(parsed_header.flags);

                parsed_flags.set_qr_indicator(true);

                if parsed_flags.opcode() == 0 {
                    parsed_flags.set_rcode(0);
                } else {
                    parsed_flags.set_rcode(4);
                };

                parsed_header.flags = parsed_flags.into();
                parsed_header.an_count = 1;

                let header_bytes = parsed_header.to_bytes();
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
