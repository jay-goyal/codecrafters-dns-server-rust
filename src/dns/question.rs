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

    pub fn parse_label(qname: &Vec<u8>, start: usize) -> (Vec<u8>, usize) {
        let mut idx = start;
        let mut domain = Vec::new();
        let mut end = start;
        let mut end_set = false;

        while qname[idx] != 0 {
            let val = qname[idx];
            if val & 0b11000000 == 0b11000000 {
                if !end_set {
                    end = idx;
                    end_set = true;
                }
                idx = (val & 0b00111111) as usize;
                continue;
            }
            domain.extend(&qname[idx..=(idx + val as usize)]);
            idx += val as usize + 1;
        }

        domain.push(0);
        if !end_set {
            end = idx;
        }

        (domain, end)
    }
}
