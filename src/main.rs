use anyhow::Result;
use codecrafters_dns_server::dns::{self, answer, header, question};
use std::env::{self};
use std::net::UdpSocket;

fn main() -> Result<()> {
    let mut args = env::args().skip(1);

    let resolver_address = args.next().and_then(|arg| {
        if arg == "--resolver" {
            args.next()
        } else {
            None
        }
    });

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {size} bytes from {source}");

                // Parse header/flags
                let mut header = header::from_bytes(buf[0..12].try_into()?);
                let mut flags = header::flags::from_bytes(&header.flags);

                // Parse questions
                let questions = question::parse(&buf[12..size]);

                // Build packets
                let mut packets = Vec::new();
                for question in questions.as_slice() {
                    let header = header.clone();
                    let packet = dns::Packet::new(header, question.clone(), None);
                    packets.push(packet);
                }

                let mut answers = forward(packets, resolver_address.as_ref(), &udp_socket)?;

                if answers.is_empty() {
                    answers = questions
                        .iter()
                        .map(|question| answer::Answer::new(&question.name))
                        .collect()
                }

                // Update header's question/answer count
                let num_of_questions = questions.len();
                let num_of_answers = answers.len();

                header.qd_count = num_of_questions as u16;
                header.an_count = num_of_answers as u16;

                // Change header to response from query
                flags.set_qr_indicator(true);
                header.flags = flags.into();

                let response = dns::Response {
                    header,
                    questions,
                    answers: Some(answers),
                };

                udp_socket
                    .send_to(&response.to_bytes(), source)
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

fn forward(
    packets: Vec<dns::Packet>,
    resolver_address: Option<&String>,
    udp_socket: &UdpSocket,
) -> Result<Vec<answer::Answer>> {
    let mut answers = Vec::<answer::Answer>::new();
    for packet in packets {
        let mut socket_buf = [0u8; 512];

        let _ = udp_socket.send_to(&packet.to_bytes(), resolver_address.unwrap());

        // If resolver only sends header break
        let size =  udp_socket.recv(&mut socket_buf)?;
        if size <= 12 {
            break;
        }

        let answer = answer::parse(&socket_buf[12..size]);
        answers.push(answer);
    }

    Ok(answers)
}
