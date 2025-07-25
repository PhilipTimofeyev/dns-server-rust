#[derive(Debug, Default, Clone)]
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

        buf[0..2].copy_from_slice(&self.packet_identifier.to_be_bytes());
        buf[2..4].copy_from_slice(&self.flags.to_be_bytes());
        buf[4..6].copy_from_slice(&self.qd_count.to_be_bytes());
        buf[6..8].copy_from_slice(&self.an_count.to_be_bytes());
        buf[8..10].copy_from_slice(&self.ns_count.to_be_bytes());
        buf[10..12].copy_from_slice(&self.ar_count.to_be_bytes());

        buf
    }
}

pub fn from_bytes(buf: [u8; 12]) -> Header {
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
pub mod flags {
    use modular_bitfield::prelude::*;

    #[bitfield]
    #[derive(Debug)]
    pub struct Flags {
        pub r_code: B4,
        pub reserved: B3,
        pub recursion_available: bool,
        pub recursion_desired: bool,
        pub truncation: bool,
        pub authoritative_answer: bool,
        pub op_code: B4,
        pub qr_indicator: bool,
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

    pub fn from_bytes(flags: &u16) -> Flags {
        let qr_indicator = (flags >> 15) != 0;
        let op_code = ((flags >> 11) & 0x0F) as u8;
        let authoritative_answer = (flags & 0x0400) != 0;
        let truncation = (flags & 0x0200) != 0;
        let recursion_desired = (flags & 0x0100) != 0;
        let recursion_available = (flags & 0x0080) != 0;
        let reserved = ((flags >> 4) & 0x07) as u8;
        let r_code = if op_code == 0 { 0 } else { 4 };

        Flags::new()
            .with_qr_indicator(qr_indicator)
            .with_op_code(op_code)
            .with_authoritative_answer(authoritative_answer)
            .with_truncation(truncation)
            .with_recursion_desired(recursion_desired)
            .with_recursion_available(recursion_available)
            .with_reserved(reserved)
            .with_r_code(r_code)
    }
}
