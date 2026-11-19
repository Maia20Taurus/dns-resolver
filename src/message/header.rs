use super::{Opcode, Qr, ResponseCode};

#[derive(Debug)]
pub struct Header {
    // id is sent by the client and mirrored in response
    id: u16,
    // qr 0 - query, 1 - response
    qr: Qr,
    opcode: Opcode,
    // authoritative answer. 1 if the responding name server is an
    // authority for the domain name in the 'Question' section.
    aa: bool,
    // truncation: 1 if the message was truncated due to excessive
    // length
    tc: bool,
    // recursion desired: set by client if they want recursive
    // resolution
    rd: bool,
    // recursion available: 1 if the server is capable of recursive
    // resolution
    ra: bool,
    // not in use, must be set to 0 (3 bits)
    z: u16,
    rcode: ResponseCode,
    // specifies the number of entries in the question section
    qdcount: u16,
    // specifies the number of RR in the answer section
    ancount: u16,
    // specifies the number of NS RRs in the authority records
    // section
    nscount: u16,
    // specifies the number of resource records in the additional
    // records section
    arcount: u16,
}

impl Header {
    /// Parse header from a DNS message
    pub fn from(stream: &[u8]) -> Result<Header, String> {
        // Parse the fields that don't make a full byte
        let bytes = u16::from_be_bytes([stream[2], stream[3]]);
        let bits: [bool; 16] = std::array::from_fn(|i| ((bytes >> (15 - i)) & 1) != 0);

        let header = Header {
            id: u16::from_be_bytes([stream[0], stream[1]]),
            qr: if bits[0] == false {
                Qr::Query
            } else {
                Qr::Response
            },
            opcode: Opcode::Query,
            aa: false,
            tc: false,
            rd: bits[7],
            ra: true,
            z: 0,
            rcode: ResponseCode::NotImplemented,
            qdcount: u16::from_be_bytes([stream[4], stream[5]]),
            ancount: u16::from_be_bytes([stream[6], stream[7]]),
            nscount: u16::from_be_bytes([stream[8], stream[9]]),
            arcount: u16::from_be_bytes([stream[10], stream[11]]),
        };

        Ok(header)
    }

    pub fn in_bytes(self) -> Result<[u8; 12], String> {
        let id_bytes = self.id.to_be_bytes();
        let qdcount_bytes = self.qdcount.to_be_bytes();
        let ancount_bytes = self.ancount.to_be_bytes();
        let nscount_bytes = self.nscount.to_be_bytes();
        let arcount_bytes = self.arcount.to_be_bytes();

        let bytes = (self.rcode.value() 
        + self.z << 4 
        + (if self.ra { 1 } else { 0 }) << 7 
        + (if self.rd { 1 } else { 0 }) << 8
        + (if self.tc {1} else {0}) << 9
        + (if self.aa {1} else {0}) << 10
        + self.opcode.value() << 11
        + self.qr.value() << 15).to_be_bytes();

        let stream = [
            id_bytes[0],
            id_bytes[1],
            bytes[0],
            bytes[1],
            qdcount_bytes[0],
            qdcount_bytes[1],
            ancount_bytes[0],
            ancount_bytes[1],
            nscount_bytes[0],
            nscount_bytes[1],
            arcount_bytes[0],
            arcount_bytes[1],
        ];

        Ok(stream)
    }

    /// Set header's response field
    pub fn set_response(&mut self, response: bool) {
        if response {
            self.qr = Qr::Response;
        } else {
            self.qr = Qr::Query;
        }
    }
}
