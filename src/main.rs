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
                let mut offset = 12;
                // println!("INTIAL: {}", buf.len());

                let mut response = Vec::<u8>::with_capacity(12);

                let mut parsed_header = header::parse_header(&buf);
                let mut parsed_flags = header::parse_flags(parsed_header.flags);

                parsed_flags.set_qr_indicator(true);

                if parsed_flags.op_code() == 0 {
                    parsed_flags.set_r_code(0);
                } else {
                    parsed_flags.set_r_code(4);
                };

                parsed_header.flags = parsed_flags.into();
                // parsed_header.an_count = 2;

                let parsed_questions = question::parse(&buf[12..]);
                let num_of_questions = parsed_questions.len();

                parsed_header.qd_count = num_of_questions as u16;
                parsed_header.an_count = num_of_questions as u16;
                let header_bytes = parsed_header.to_bytes();
                response.extend_from_slice(&header_bytes);
                
                println!("Header {:?}", parsed_header);
                

                for question in parsed_questions.as_slice() {
                    println!("Question{:?}", question.to_bytes());
                    response.extend_from_slice(&question.to_bytes());
                }
                for question in parsed_questions.as_slice() {

                    let mut answer = answer::Answer::new(&question.name.to_vec());
                    println!("question name {:?}", question.name);
                    // answer.name = question.name.to_vec();
                    // answer.data  = 0;

                    let answer_bytes = answer.to_bytes();

                    response.extend_from_slice(&answer_bytes);
                }
                println!("FINAL: {:?}", response);
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
