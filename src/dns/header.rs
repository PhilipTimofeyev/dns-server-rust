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
