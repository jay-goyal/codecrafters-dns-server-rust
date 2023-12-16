use super::header::Header;

pub struct Message {
    header: Header,
}

impl Message {
    pub fn new(header: Header) -> Message {
        Message { header }
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.header.as_bytes()
    }
}
