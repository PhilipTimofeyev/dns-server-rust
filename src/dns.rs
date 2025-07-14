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
