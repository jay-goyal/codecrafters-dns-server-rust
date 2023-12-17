use super::{header::Header, question::Question};

pub struct Message {
    header: Header,
    question: Question,
}

impl Message {
    pub fn new(header: Header, question: Question) -> Message {
        Message { header, question }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(self.header.as_bytes());
        bytes.extend(self.question.as_bytes());

        bytes
    }
}
