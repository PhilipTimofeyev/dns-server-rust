use bincode::Encode;
use modular_bitfield::prelude::*;

#[derive(Debug, Default, Encode)]
pub struct Header {
    pub packet_identifier: u16,
    pub flags: u16,
    pub qd_count: u16,
    pub an_count: u16,
    pub ns_count: u16,
    pub ar_count: u16,
}

impl Header {
    pub fn to_bytes(&self) -> [u8; 12] {
        let mut buf = [0; 12];

        //Extract high and low byte
        buf[0] = (self.packet_identifier >> 8) as u8;
        buf[1] = (self.packet_identifier & 0xFF) as u8;
        buf[2] = (self.flags >> 8) as u8;
        buf[3] = (self.flags & 0xFF) as u8;
        buf[4] = (self.qd_count >> 8) as u8;
        buf[5] = (self.qd_count & 0xFF) as u8;
        buf[6] = (self.an_count >> 8) as u8;
        buf[7] = (self.an_count & 0xFF) as u8;
        buf[8] = (self.ns_count >> 8) as u8;
        buf[9] = (self.ns_count & 0xFF) as u8;
        buf[10] = (self.ar_count >> 8) as u8;
        buf[11] = (self.ar_count & 0xFF) as u8;

        buf
    }
}

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

// #[derive(Debug, Default, Encode)]
// pub struct Question {
//     pub name: Vec<u8>,
//     pub record_type: u16,
//     pub class: u16,
// }

// impl Question {
//     pub fn new(domain: String) -> Self {
//         let mut buf = Vec::new();
//         let a = domain.split('.');
//         for label in a {
//             buf.push(label.len() as u8);
//             buf.extend_from_slice(label.as_bytes());
//         }
//
//         Question {
//             name: buf,
//             record_type: 1,
//             class: 1,
//         }
//     }
//
//     pub fn to_bytes(&self) -> Vec<u8> {
//         let mut buf = Vec::new();
//
//         buf.extend_from_slice(&self.name);
//         buf.push(0);
//         buf.extend_from_slice(&self.record_type.to_be_bytes());
//         buf.extend_from_slice(&self.class.to_be_bytes());
//
//         buf
//
//     }
// }

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
