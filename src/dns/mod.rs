pub mod answer;
pub mod header;
pub mod question;

#[derive(Debug)]
pub struct DnsPacket {
    pub header: header::Header,
    pub questions: Vec<question::Question>,
    pub answers: Option<Vec<answer::Answer>>,
}

impl DnsPacket {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.header.to_bytes());

        for question in &self.questions {
            bytes.extend_from_slice(&question.to_bytes());
        }

        if let Some(answers) = &self.answers {
            for answer in answers {
                bytes.extend_from_slice(&answer.to_bytes());
            }
        }

        bytes
    }
}
