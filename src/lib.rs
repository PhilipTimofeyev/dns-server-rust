pub mod dns;
use anyhow::Result;
use std::env::{self};
use std::net::UdpSocket;

pub fn run_server() -> Result<()> {
    let mut args = env::args().skip(1);

    let resolver_address = args.next().and_then(|arg| {
        if arg == "--resolver" {
            args.next()
        } else {
            None
        }
    });

    println!("Server running:");
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {size} bytes from {source}");

                // Parse header/flags
                let mut header = dns::header::from_bytes(buf[0..12].try_into()?);
                let mut flags = dns::header::flags::from_bytes(&header.flags);

                // Parse questions
                let questions = dns::question::parse(&buf[12..size]);

                // Build packets
                let mut packets = Vec::new();
                for question in questions.as_slice() {
                    let header = header.clone();
                    let packet = dns::Packet::new(header, question, None);
                    packets.push(packet);
                }

                // Update header's question/answer count
                header.qd_count = packets.len() as u16;
                header.an_count = packets.len() as u16;

                forward(&mut packets, resolver_address.as_ref(), &udp_socket)?;

                // Update header to response from query
                flags.set_qr_indicator(true);
                header.flags = flags.into();

                let response = dns::Response::new(header, &packets);

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
    packets: &mut [dns::Packet],
    resolver_address: Option<&String>,
    udp_socket: &UdpSocket,
) -> Result<()> {
    for packet in packets.iter_mut() {
        let mut resolver_buf = [0u8; 512];
        let mut size = 0;

        if resolver_address.is_some() {
            let _ = udp_socket.send_to(&packet.to_bytes(), resolver_address.unwrap());
            size = udp_socket.recv(&mut resolver_buf)?;
        }

        // If resolver only sends header, build answers
        packet.answer = if size <= 12 {
            let answer = dns::answer::Answer::new(&packet.question.name);
            Some(answer)
        } else {
            let answer = dns::answer::parse(&resolver_buf[12..size]);
            Some(answer)
        }
    }

    Ok(())
}
