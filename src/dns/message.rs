use super::{answer::Answer, header::Header, question::Question};

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
