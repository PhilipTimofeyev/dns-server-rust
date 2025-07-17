use modular_bitfield::prelude::*;

#[derive(Debug, Default)]
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

pub fn parse_header(buf: &[u8]) -> Header {
    let packet_identifier = u16::from_be_bytes([buf[0], buf[1]]);
    let flags = u16::from_be_bytes([buf[2], buf[3]]);
    let qd_count = u16::from_be_bytes([buf[4], buf[5]]);
    let an_count = u16::from_be_bytes([buf[6], buf[7]]);
    let ns_count = u16::from_be_bytes([buf[8], buf[9]]);
    let ar_count = u16::from_be_bytes([buf[10], buf[11]]);

    Header {
        packet_identifier,
        flags,
        qd_count,
        an_count,
        ns_count,
        ar_count,
    }
}

pub fn parse_flags(flags: u16) -> Flags {
    let qr_indicator = ((flags >> 15) & 1) as u8;
    let op_code = ((flags >> 11) & 0x0F) as u8;
    let authoritative_answer = ((flags & 0x0400) != 0) as u8;
    let truncation = ((flags & 0x0200) != 0) as u8;
    let recursion_desired = (flags & 0x0100) != 0;
    let recursion_available = ((flags & 0x0080) != 0) as u8;
    let reserved = ((flags >> 4) & 0x07) as u8;
    let rcode = (flags & 0x000F) as u8;

    Flags::new()
        .with_qr_indicator(qr_indicator)
        .with_opcode(op_code)
        .with_authoritative_answer(authoritative_answer)
        .with_truncation(truncation)
        .with_recursion_desired(recursion_desired as u8)
        .with_recursion_available(recursion_available)
        .with_reserved(reserved)
        .with_rcode(rcode)
}
