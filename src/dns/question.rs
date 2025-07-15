#[derive(Debug, Default)]
pub struct Question {
    pub name: Vec<u8>,
    pub record_type: u16,
    pub class: u16,
}

impl Question {
    pub fn new(domain: String) -> Self {
        let mut buf = Vec::new();
        let a = domain.split('.');
        for label in a {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }

        Question {
            name: buf,
            record_type: 1,
            class: 1,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend_from_slice(&self.name);
        buf.push(0);
        buf.extend_from_slice(&self.record_type.to_be_bytes());
        buf.extend_from_slice(&self.class.to_be_bytes());

        buf

    }
}
