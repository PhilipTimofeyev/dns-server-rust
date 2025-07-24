use anyhow::Result;
use codecrafters_dns_server::dns::{self, answer, header, question};
use std::env::{self, args};
use std::net::{Ipv4Addr, SocketAddr, UdpSocket};

fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let mut args = env::args();
    args.next();

    let address = if let Some(resolver) = args.next() {
        if resolver == "--resolver" {
            let address = args.next().unwrap();
            // let address: SocketAddr = address.parse().unwrap();
            Some(address)
            // println!("HERE {}", address);
        } else {
            None
        }
    } else {
        None
    };

    // let socket = UdpSocket::bind(address.unwrap()).expect("couldn't bind to address");

    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];
    // let mut hmm = Vec::new();

    // udp_socket
    //     .send_to(&[0; 10], "127.0.0.1:4242")
    //     .expect("couldn't send data");

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {size} bytes from {source}");
                let mut answers: Vec<answer::Answer> = Vec::new();

                // let mut response = Vec::<u8>::with_capacity(12);

                // Parse header
                let mut parsed_header = header::from_bytes(buf[0..12].try_into()?);

                // Parse questions
                let parsed_questions = question::parse(&buf[12..]);
                let num_of_questions = parsed_questions.len();

                // Build header
                // let header_bytes = parsed_header.to_bytes();
                // response.extend_from_slice(&header_bytes);

                // Build questions
                // All questions must be built first, then answers
                // for question in parsed_questions.as_slice() {
                //     response.extend_from_slice(&question.to_bytes());
                // }
                // Build answers
                // for question in parsed_questions.as_slice() {
                // let ip_address = Ipv4Addr::new(12, 12, 12, 12);
                //     let answer = answer::Answer::new(&question.name, ip_address);
                //     // response.extend_from_slice(&answer.to_bytes());
                // }

                // let answers: Vec<answer::Answer> = parsed_questions
                //     .iter()
                //     .map(|question| answer::Answer::new(&question.name, ip_address))
                //     .collect();

                let mut answers =
                    forward(
                        parsed_header.clone(),
                        parsed_questions.clone(),
                        address.clone(),
                        &udp_socket,
                    )?;
                

                if answers.is_empty() {
                    answers = parsed_questions
                        .iter()
                        .map(|question| answer::Answer::new(&question.name))
                        .collect()
                }

                println!("ANSWERS{:?}", answers);

                // Update headers question/answer count
                parsed_header.qd_count = num_of_questions as u16;
                parsed_header.an_count = num_of_questions as u16;
                let mut parsed_flags = header::flags::from_bytes(parsed_header.flags);
                // parsed_header.packet_identifier = 1234;

                // Update header's flags
                parsed_flags.set_qr_indicator(true);
                parsed_header.flags = parsed_flags.into();

                let response = dns::DnsPacket {
                    header: parsed_header,
                    questions: parsed_questions,
                    answers: Some(answers),
                };

                // println!("RESPONSE {:?}", &response.to_bytes());

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
    header: header::Header,
    questions: Vec<question::Question>,
    address: Option<String>,
    udp_socket: &UdpSocket,
) -> Result<Vec<answer::Answer>> {
    let mut answers = Vec::<answer::Answer>::new();
    for question in questions {
        let packet = dns::DnsPacket {
            header: header.clone(),
            questions: vec![question.clone()],
            answers: None,
        };
        // println!("{:?}", packet.to_bytes());
        // udp_socket.set_read_timeout(Some(Duration::from_secs(2)))?;

        let sent = udp_socket.send_to(&packet.to_bytes(), address.clone().unwrap());
        let mut hmm = [0u8; 512];
        udp_socket.recv(&mut hmm)?;
        // println!("Sent: {:?}", sent);
        if hmm[13] == 0 {
            break;
        }
        let answer = answer::parse(&hmm[12..]);
        // udp_socket.recv(&mut hmm);
        // println!("Hmm {:?}", hmm);
        answers.push(answer.clone());
        // println!("Answer {:?}", answer);
    }

    Ok(answers)
}
