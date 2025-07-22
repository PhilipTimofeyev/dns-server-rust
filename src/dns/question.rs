use std::io::Cursor;
use std::io::{BufRead, Read};

#[derive(Debug, Default)]
pub struct Question {
    pub name: Vec<u8>,
    pub record_type: u16,
    pub class: u16,
}

impl Question {
    // pub fn new(domain: String) -> Self {
    //     let mut buf = Vec::new();
    //     let a = domain.split('.');
    //     for label in a {
    //         buf.push(label.len() as u8);
    //         buf.extend_from_slice(label.as_bytes());
    //         buf.push(0);
    //     }
    //
    //     Question {
    //         name: buf,
    //         record_type: 1,
    //         class: 1,
    //     }
    // }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        buf.extend_from_slice(&self.name);
        buf.extend_from_slice(&self.record_type.to_be_bytes());
        buf.extend_from_slice(&self.class.to_be_bytes());

        buf
    }
}

pub fn parse(bytes: &[u8]) -> Vec<Question> {
    let len = bytes.len();
    let mut cursor = Cursor::new(bytes);
    let mut buf = vec![];
    let mut temp = [0u8; 4];
    let mut result: Vec<Question> = Vec::new();

    while cursor.position() < len as u64 {
        // read until null byte after domain name
        let _ = cursor.read_until(0, &mut buf);
        // read the four bytes of type and class
        let _ = cursor.read_exact(&mut temp);

        // if bytes are null then reached end of message
        if temp.iter().all(|n| *n == 0) {
            break;
        }

        let mut question = Question {
            name: buf.clone(),
            record_type: 1,
            class: 1,
        };

        // If question has compressed label sequence, use label sequence of previous question
        if buf.contains(&0xc0) {
            let last_question = result
                .iter()
                .rfind(|question| !question.name.contains(&0xc0))
                .unwrap();
            question.name = last_question.name.clone();
        }

        result.push(question);
        buf.clear();
    }

    result
}
