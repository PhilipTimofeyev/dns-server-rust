use std::io::Cursor;
use std::io::{self, BufRead, BufReader, Read};

use bytes::buf;

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

// pub fn parse(bytes: &[u8]) -> Question {
//     let domain_name_end_idx = bytes.iter().position(|&byte| byte == 0).unwrap();
//     let hmm = bytes.split(|&byte| byte == 0);
//     let domain_name = &bytes[12..(domain_name_end_idx + 13)];
//
//     Question {
//         name: domain_name.to_vec(),
//         record_type: 1,
//         class: 1,
//     }
// }

pub fn parse(bytes: &[u8]) -> Vec<Question> {
    // println!("Questions: {:?}", bytes);
    let len = bytes.len();
    let mut cursor = Cursor::new(bytes);
    let mut buf = vec![];
    let mut temp = [0u8; 4];
    let mut result: Vec<Question> = Vec::new();

    while cursor.position() < len as u64 {
        cursor.read_until(0, &mut buf);
        println!("buf: {:?}", buf);
        cursor.read_exact(&mut temp);
        if temp.iter().all(|n| *n == 0) {
            break;
        }

        // buf.extend_from_slice(&temp);
        let mut question = Question {
            name: buf.clone(),
            record_type: 1,
            class: 1,
        };
        if buf.contains(&192) {
            question.name = result[0].name.clone();
        }
        result.push(question);
        buf.clear();
        //     // println!("Cursor : {}",cursor.position());
        //     // break;
    }

    // println!("Result: {:?}", result);
    // printreln!("Buf {:?}", buf);
    // println!("{:?}", cursor);
    // let hmm = bytes.split(|&byte| byte == 0);
    // let domain_name = &bytes[12..(domain_name_end_idx + 13)];
    result
}

// let a: Vec<String> = question_bytes
//     .split(|byte| !byte.is_ascii_alphanumeric())
//     .filter(|bytes| !bytes.is_empty())
//     .map(|bytes| str::from_utf8(bytes).unwrap().to_string())
//     .collect();
