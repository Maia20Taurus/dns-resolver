use crate::RecordType;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassType {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

impl TryFrom<u16> for ClassType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ClassType::IN),
            2 => Ok(ClassType::CS),
            3 => Ok(ClassType::CH),
            4 => Ok(ClassType::HS),
            _ => Err("Unknown DNS class type"),
        }
    }
}

impl From<ClassType> for u16 {
    fn from(class: ClassType) -> Self {
        class as u16
    }
}

#[derive(Debug)]
pub struct Question {
    // domain name e.g. example.com.
    qname: String,
    // the type of the query, specifying which kind of record is
    // being asked for (e.g. A or MX)
    qtype: RecordType,
    // the class of the query (IN or CN)
    qclass: ClassType,
}

impl Question {
    /// Construct question from a stream.
    /// Expects `stream` to be the entire DNS query (not just the slice
    /// that includes the `Question` section)
    /// If Ok, will return the new Question and the slice of the remaining query
    /// after the question
    pub fn from(stream: &[u8]) -> Result<(Question, &[u8]), String> {
        let question_onwards = &stream[12..];
        let mut qname_iter = question_onwards.iter();
        let mut domain = String::new();

        // Track the length of the domain
        let mut domain_length_octets = 0;
        loop {
            let label_length = *qname_iter.next().expect("No length found.");

            if label_length == 0 {
                break;
            }

            for _ in 0..label_length {
                let parsed = [*qname_iter.next().expect("Couldn't parse octet")];
                let letter = std::str::from_utf8(&parsed).expect("Couldn't parse letter");
                domain.push_str(letter);

                domain_length_octets += 1;
            }
            domain.push_str(".");
            domain_length_octets += 1;
        }
        // By this point we should have the domain

        let after_domain = &question_onwards[domain_length_octets + 1..];

        let qtype = RecordType::try_from(u16::from_be_bytes([after_domain[0], after_domain[1]]))?;
        let qclass = ClassType::try_from(u16::from_be_bytes([after_domain[2], after_domain[3]]))?;

        Ok((
            Question {
                qname: domain,
                qtype,
                qclass,
            },
            &after_domain[4..],
        ))
    }

    /// Construct bytes from a Question
    pub fn to_be_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();

        let mut label: Vec<u8> = Vec::new();
        for b in self.qname.chars() {
            match b {
                '.' => {
                    bytes.push(label.len() as u8);
                    bytes.append(&mut label)
                }
                _ => label.push(b as u8),
            }
        }

        let qtype_bytes = u16::from(self.qtype).to_be_bytes();
        let qclass_bytes = u16::from(self.qclass).to_be_bytes();

        bytes.extend(qtype_bytes);
        bytes.extend(qclass_bytes);

        bytes
    }
}
