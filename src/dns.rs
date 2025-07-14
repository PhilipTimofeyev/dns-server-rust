use modular_bitfield::prelude::*;
use bincode::Encode;

#[derive(Debug, Default, Encode)]
pub struct Header {
    pub packet_identifier: u16,
    pub flags: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

// impl From<Header> for [u8; 12] {
//     fn from(header: Header) -> [u8; 12] {
//         let mut buffer = [0u8; 12];
//
//         buffer[0..2].copy_from_slice(&header.packet_identifier.to_be_bytes());
//         buffer[2..4].copy_from_slice(&header.flags.to_be_bytes());
//         buffer[4..6].copy_from_slice(&header.qd_count.to_be_bytes());
//         buffer[6..8].copy_from_slice(&header.an_count.to_be_bytes());
//         buffer[8..10].copy_from_slice(&header.ns_count.to_be_bytes());
//         buffer[10..12].copy_from_slice(&header.ar_count.to_be_bytes());
//
//         buffer
//     }
// }

#[bitfield]
#[derive(Debug)]
pub struct Flags {
    pub rcode: B4,
    pub reserved: B3,
    pub recursion_available: B1,
    pub recursion_desired: B1,
    pub truncation: B1,
    pub authoritative_answer: B1,
    pub opcode: B4,
    pub qr_indicator: B1,
}

impl Default for Flags {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Flags> for u16 {
    fn from(flags: Flags) -> u16 {
        u16::from_le_bytes(flags.into_bytes())
    }
}
