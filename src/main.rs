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

                let flags = dns::Flags::default().with_qr_indicator(1).into();

                let header = dns::Header {
                    packet_identifier: 1234,
                    flags,
                    ..dns::Header::default()
                };

                let response = bincode::encode_to_vec(header, bincode::config::standard())?;

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


