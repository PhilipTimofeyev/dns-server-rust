pub struct Answer {
    name: Vec<u8>,
    answer_type: u16,
    class: u16,
    ttl: u32,
    length: u16,
    data: u32
}

impl Answer {
    pub fn new(domain: String) -> Self {
        let mut buf = Vec::new();
        let a = domain.split('.');
        for label in a {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0);

        let data: [u8; 4] = [0x08,0x08,0x08,0x08];
        let length = data.len() as u16;

        Answer {
            name: buf,
            answer_type: 1,
            class: 1,
            ttl: 60,
            length,
            data: u32::from_be_bytes(data)
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend_from_slice(&self.name);
        buf.extend_from_slice(&self.answer_type.to_be_bytes());
        buf.extend_from_slice(&self.class.to_be_bytes());
        buf.extend_from_slice(&self.ttl.to_be_bytes());
        buf.extend_from_slice(&self.length.to_be_bytes());
        buf.extend_from_slice(&self.data.to_be_bytes());

        buf
    }
}
