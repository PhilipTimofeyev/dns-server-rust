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

                let mut response = Vec::<u8>::with_capacity(12);

                // Parse header
                let mut parsed_header = header::parse_header(&buf);
                let mut parsed_flags = header::flags::parse(parsed_header.flags);

                // Update header's flags
                parsed_flags.set_qr_indicator(true);
                parsed_header.flags = parsed_flags.into();

                // Parse questions
                let parsed_questions = question::parse(&buf[12..]);
                let num_of_questions = parsed_questions.len();

                // Update headers question/answer count
                parsed_header.qd_count = num_of_questions as u16;
                parsed_header.an_count = num_of_questions as u16;

                // Build header
                let header_bytes = parsed_header.to_bytes();
                response.extend_from_slice(&header_bytes);

                // Build questions
                // All questions must be built first, then answers
                for question in parsed_questions.as_slice() {
                    response.extend_from_slice(&question.to_bytes());
                }
                // Build answers
                for question in parsed_questions.as_slice() {
                    let answer = answer::Answer::new(&question.name.to_vec());
                    response.extend_from_slice(&answer.to_bytes());
                }

                udp_socket
                    .send_to(&response, source)
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
