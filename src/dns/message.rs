use super::{answer::Answer, header::Header, question::Question};
use std::net::Ipv4Addr;

pub struct Message {
    header: Header,
    questions: Vec<Question>,
    answers: Vec<Answer>,
}

impl Message {
    pub fn new(header: Header, questions: Vec<Question>, answers: Vec<Answer>) -> Message {
        Message {
            header,
            questions,
            answers,
        }
    }

    pub fn gen_response(message: &Vec<u8>, _size: usize) -> Message {
        let pid = ((message[0] as u16) << 8) + message[1] as u16;
        let opcode = (message[2] & 0b01111000) >> 3;
        let rd = message[2] & 0b00000001;
        let rcode;
        let qcount = ((message[4] as u16) << 8) + message[5] as u16;

        if opcode == 0 {
            rcode = 0;
        } else {
            rcode = 4;
        }

        let mut idx = 12;
        let mut domains = Vec::new();
        for _ in 0..qcount {
            let val = Question::parse_label(message, idx);
            domains.push(val.0);
            idx = val.1 + 3;
        }

        let mut questions = Vec::new();
        let mut answers = Vec::new();

        for domain in domains {
            questions.push(Question::new(domain.clone(), 1, 1));
            answers.push(Answer::new(
                domain,
                1,
                1,
                60,
                4,
                Ipv4Addr::new(8, 8, 8, 8).octets().to_vec(),
            ));
        }

        let header = Header::new(
            pid,
            1,
            opcode,
            0,
            0,
            rd,
            0,
            0,
            rcode,
            questions.len() as u16,
            answers.len() as u16,
            0,
            0,
        );
        let message = Message::new(header, questions, answers);

        message
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.header.as_bytes());
        self.questions
            .iter()
            .for_each(|question| bytes.extend(question.as_bytes()));
        self.answers
            .iter()
            .for_each(|answer| bytes.extend(answer.as_bytes()));

        bytes
    }
}
