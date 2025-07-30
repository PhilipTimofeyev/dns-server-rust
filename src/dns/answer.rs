use std::io::{BufRead, Read};
use std::io::{Cursor, Seek};

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
    let mut cursor = Cursor::new(bytes);
    let mut header = [0u8; 12];
    let mut temp = [0u8; 4];
    let mut name = Vec::new();
    let mut compressed = [0u8; 2];
    let mut answer_type = [0u8; 2];
    let mut class = [0u8; 2];
    let mut ttl = [0u8; 4];
    let mut length = [0u8; 2];

    println!("Bytes {:?}", bytes);
    let _ = cursor.read_exact(&mut header);
    println!("Header {:?}", header);
    let _ = cursor.skip_until(0);
    let _ = cursor.read_exact(&mut temp);
    println!("Temp {:?}", temp);
    // println!("temp {:?}", cursor.position());
    let _ = cursor.read_until(0, &mut name);
    if name.contains(&0xc0) {
        cursor.seek_relative(-3);
        let _ = cursor.read_exact(&mut compressed);
        name = compressed.to_vec();
    }
    println!("Name {:?}", name);
    // println!("cursor {:?}", bytes[cursor.position() as usize]);
    let _ = cursor.read_exact(&mut answer_type);
    println!("Answer Type {:?}", answer_type);
    let _ = cursor.read_exact(&mut class);
    println!("CLass Type {:?}", class);
    let _ = cursor.read_exact(&mut ttl);
    println!("TTl Type {:?}", ttl);
    let _ = cursor.read_exact(&mut length);
    println!("Length {:?}", length);

    let data = &bytes[cursor.position() as usize..];

    // println!("bytes {:?}", bytes);
    println!("DATA {:?}", data);

    Answer {
        name,
        answer_type: u16::from_be_bytes(answer_type),
        class: u16::from_be_bytes(class),
        ttl: u32::from_be_bytes(ttl),
        length: u16::from_be_bytes(length),
        data: u32::from_be_bytes(data.try_into().expect("Address must be 4 bytes")),
    }
}
