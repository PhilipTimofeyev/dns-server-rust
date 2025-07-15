use anyhow::Result;
use codecrafters_dns_server::dns;
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

                let flags = dns::Flags::default().with_qr_indicator(1).into();

                let header = dns::Header {
                    packet_identifier: 1234,
                    qd_count: 1,
                    flags,
                    ..dns::Header::default()
                };

                let header_bytes = header.to_bytes();

                offset += header_bytes.len();
                buf[0..offset].copy_from_slice(&header_bytes);

                // // buf[0..10].copy_from_slice(&header_bytes);
                // result.extend_from_slice(&header_bytes);

                // let question = dns::Question::new("codecrafters.io".to_string());

                // let questions = dns::Question {
                //     name: "google".to_string(),
                //     record_type: 1,
                //     class: 1,
                // };
                //
                // let length = result.len();

                // let question_bytes = bincode::encode_to_vec(questions, bincode::config::standard())?;
                // result[0..length].copy_from_slice(&question.to_bytes());
                // let question_bytes = question.to_bytes();
                // buf[offset..offset + question_bytes.len()].copy_from_slice(&question_bytes);

                // buf[offset..offset + question_bytes.len()].copy_from_slice(&result);

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
