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
            buf.push(0);
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
        buf.extend_from_slice(&self.record_type.to_be_bytes());
        buf.extend_from_slice(&self.class.to_be_bytes());

        buf
    }
}

pub fn parse(bytes: &[u8]) -> Question {
    let domain_name_end_idx = bytes[12..].iter().position(|&byte| byte == 0).unwrap();
    let domain_name = &bytes[12..(domain_name_end_idx + 13)];

    Question {
        name: domain_name.to_vec(),
        record_type: 1,
        class: 1,
    }
}

// let a: Vec<String> = question_bytes
//     .split(|byte| !byte.is_ascii_alphanumeric())
//     .filter(|bytes| !bytes.is_empty())
//     .map(|bytes| str::from_utf8(bytes).unwrap().to_string())
//     .collect();
