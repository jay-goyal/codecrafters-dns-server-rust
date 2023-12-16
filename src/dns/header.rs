pub struct Header {
    bytes: [u8; 12],
}

impl Header {
    pub fn new(
        pid: u16,
        qr: u8,
        opcode: u8,
        aa: u8,
        tc: u8,
        rd: u8,
        ra: u8,
        z: u8,
        rcode: u8,
        qdcount: u16,
        ancount: u16,
        nscount: u16,
        arcount: u16,
    ) -> Header {
        Header {
            bytes: [
                (pid >> 8) as u8,
                pid as u8,
                (qr << 7) | (opcode << 3) | (aa << 2) | (tc << 1) | rd,
                (ra << 7) | (z << 4) | rcode,
                (qdcount >> 8) as u8,
                qdcount as u8,
                (ancount >> 8) as u8,
                ancount as u8,
                (nscount >> 8) as u8,
                nscount as u8,
                (arcount >> 8) as u8,
                arcount as u8,
            ],
        }
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
