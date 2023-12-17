pub struct Question {
    qname: Vec<u8>,
    qtype: [u8; 2],
    qclass: [u8; 2],
}

impl Question {
    pub fn new(qname: Vec<u8>, qtype: u16, qclass: u16) -> Question {
        Question {
            qname,
            qtype: qtype.to_be_bytes(),
            qclass: qclass.to_be_bytes(),
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.qname.clone();
        bytes.extend(self.qtype);
        bytes.extend(self.qclass);

        bytes
    }
}
