pub mod answer;
pub mod header;
pub mod question;

#[derive(Debug)]
pub struct Packet <'a> {
    pub header: header::Header,
    pub question: &'a question::Question,
    pub answer: Option<answer::Answer>,
}

impl <'a> Packet <'a> {
    pub fn new(
        header: header::Header,
        question: &'a question::Question,
        answer: Option<answer::Answer>,
    ) -> Self {
        Packet {
            header,
            question,
            answer,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(&self.header.to_bytes());
        bytes.extend_from_slice(&self.question.to_bytes());

        if let Some(answer) = &self.answer {
            bytes.extend_from_slice(&answer.to_bytes());
        }

        bytes
    }
}

#[derive(Debug)]
pub struct Response <'a> {
    pub header: header::Header,
    pub questions: Vec<&'a question::Question>,
    pub answers: Option<Vec<&'a answer::Answer>>,
}

impl <'a> Response <'a> {
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

    pub fn new(header: header::Header, packets: &'a Vec<Packet>) -> Self {
        let mut questions = Vec::new();
        let mut answers = Vec::new();

        for packet in packets {
            questions.push(packet.question);
            if let Some(answer) = &packet.answer {
                answers.push(answer);
            }
        }

        Response {
            header,
            questions,
            answers: Some(answers),
        }
    }
}
