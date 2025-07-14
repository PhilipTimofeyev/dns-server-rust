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

                let response = dns::Header {
                    packet_identifier: 1234,
                    flags,
                    ..dns::Header::default()
                };

                let header_buf = build_header_be_bytes_buf(response);

                let response = bincode::encode_to_vec(header_buf, bincode::config::standard())?;
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

fn build_header_be_bytes_buf(response: dns::Header) -> [u8; 12] {
    let mut buffer = [0u8; 12];

    buffer[0..2].copy_from_slice(&response.packet_identifier.to_be_bytes());
    buffer[2..4].copy_from_slice(&response.flags.to_be_bytes());
    buffer[4..6].copy_from_slice(&response.qd_count.to_be_bytes());
    buffer[6..8].copy_from_slice(&response.an_count.to_be_bytes());
    buffer[8..10].copy_from_slice(&response.ns_count.to_be_bytes());
    buffer[10..12].copy_from_slice(&response.ar_count.to_be_bytes());

    buffer
}
