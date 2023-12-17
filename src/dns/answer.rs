pub struct Answer {
    aname: Vec<u8>,
    atype: [u8; 2],
    aclass: [u8; 2],
    attl: [u8; 4],
    rdlength: [u8; 2],
    rdata: Vec<u8>,
}

impl Answer {
    pub fn new(
        dname: String,
        qtype: u16,
        qclass: u16,
        attl: u32,
        rdlength: u16,
        rdata: Vec<u8>,
    ) -> Answer {
        let mut qname = Vec::new();

        dname.split(".").for_each(|l| {
            qname.push(l.len() as u8);
            qname.extend(l.bytes());
        });
        qname.push(0);

        Answer {
            aname: qname,
            atype: qtype.to_be_bytes(),
            aclass: qclass.to_be_bytes(),
            attl: attl.to_be_bytes(),
            rdlength: rdlength.to_be_bytes(),
            rdata,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.aname.clone();
        bytes.extend(self.atype);
        bytes.extend(self.aclass);
        bytes.extend(self.attl);
        bytes.extend(self.rdlength);
        bytes.extend(self.rdata.iter());

        bytes
    }
}
