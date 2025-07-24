use std::io::Cursor;
use std::io::{BufRead, Read};
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
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

        // let data: [u8; 4] = [0x08, 0x08, 0x08, 0x08];
        // let length = data.len() as u16;

        Answer {
            name: buf.to_vec(),
            answer_type: 1,
            class: 1,
            ttl: 60,
            length: 4,
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

pub fn parse(bytes: &[u8]) -> Answer {
    // println!("BYTES {:?}", bytes);
    let len = bytes.len();
    let mut cursor = Cursor::new(bytes);
    let mut hmm = [0u8; 4];
    let mut name = vec![];
    let mut answer_type = [0u8; 2];
    let mut class = [0u8; 2];
    let mut ttl = [0u8; 4];
    let mut length = [0u8; 2];

    // let mut result: Vec<Question> = Vec::new();

    // while cursor.position() < len as u64 {
    // read until null byte after domain name
    let _ = cursor.skip_until(0);
    let _ = cursor.read_exact(&mut hmm);
    // read the four bytes of type and class
    let _ = cursor.read_until(0, &mut name);
    let _ = cursor.read_exact(&mut answer_type);
    let _ = cursor.read_exact(&mut class);
    let _ = cursor.read_exact(&mut ttl);
    let _ = cursor.read_exact(&mut length);

    // println!("Cursor position {:?}", cursor.position());

    // if bytes are null then reached end of message
    // if temp.iter().all(|n| *n == 0) {
    //     break;
    // }

    // let mut answer = Answer {
    //     name: buf.clone(),
    //     record_type: 1,
    //     class: 1,
    // };

    // If question has compressed label sequence, use label sequence of previous question
    // if buf.contains(&0xc0) {
    //     let last_question = result
    //         .iter()
    //         .rfind(|question| !question.name.contains(&0xc0))
    //         .unwrap();
    //     question.name = last_question.name.clone();
    // }

    let end_of_bytes = bytes.iter().rposition(|&b| b != 0).unwrap();
    println!("{end_of_bytes}");

    let data = &bytes[cursor.position() as usize..=end_of_bytes];
    // println!("DATA {:?}", data);

    Answer {
        name,
        answer_type: u16::from_be_bytes(answer_type),
        class: u16::from_be_bytes(class),
        ttl: u32::from_be_bytes(ttl),
        length: u16::from_be_bytes(length),
        data: u32::from_be_bytes(data.try_into().expect("slice must be 4 bytes")),
    }

    // buf.extend_from_slice(&bytes[cursor.position() as usize..=end_of_bytes]);
    // buf.clear();
    // }
    // buf
}
