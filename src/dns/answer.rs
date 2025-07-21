#[derive(Debug)]
pub struct Answer {
    pub name: Vec<u8>,
    answer_type: u16,
    class: u16,
    ttl: u32,
    length: u16,
    pub data: u32,
}

impl Answer {
    pub fn new(buf: &[u8]) -> Self {
        // let mut buf = Vec::new();
        // let a = domain.split('.');
        // for label in a {
        //     buf.push(label.len() as u8);
        //     buf.extend_from_slice(label.as_bytes());
        // }
        // buf.push(0);

        let data: [u8; 4] = [0x08, 0x08, 0x08, 0x08];
        let length = data.len() as u16;

        Answer {
            name: buf.to_vec(),
            answer_type: 1,
            class: 1,
            ttl: 60,
            length,
            data: 0,
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

impl Default for Answer {
    fn default() -> Self {
        Answer {
            name: Vec::default(),
            answer_type: 1,
            class: 1,
            ttl: 60,
            length: 0,
            data: 0,
        }
    }
}
