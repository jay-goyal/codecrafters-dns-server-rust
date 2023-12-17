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

    pub fn gen_response(message: &[u8], _size: usize) -> Message {
        let pid = ((message[0] as u16) << 8) + message[1] as u16;
        let opcode = message[3] & 0b01111000;
        let rd = message[3] & 0b00000001;
        let rcode;

        if opcode == 0 {
            rcode = 0;
        } else {
            rcode = 4;
        }

        let header = Header::new(pid, 1, opcode, 0, 0, rd, 0, 0, rcode, 1, 1, 0, 0);
        let questions = vec![Question::new("codecrafters.io".to_string(), 1, 1)];
        let answers = vec![Answer::new(
            "codecrafters.io".to_string(),
            1,
            1,
            60,
            4,
            Ipv4Addr::new(8, 8, 8, 8).octets().to_vec(),
        )];
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
