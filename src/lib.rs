pub mod message;
pub mod resource_record;

#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    NULL = 10,
    PTR = 12,
    HINFO = 13,
    MINFO = 14,
    MX = 15,
    TXT = 16,
}

impl TryFrom<u16> for RecordType {
    type Error = &'static str;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        println!("{}", value);
        match value {
            1 => Ok(RecordType::A),
            2 => Ok(RecordType::NS),
            5 => Ok(RecordType::CNAME),
            6 => Ok(RecordType::SOA),
            10 => Ok(RecordType::NULL),
            12 => Ok(RecordType::PTR),
            13 => Ok(RecordType::HINFO),
            14 => Ok(RecordType::MINFO),
            15 => Ok(RecordType::MX),
            16 => Ok(RecordType::TXT),
            _ => Err("Unknown DNS record type"),
        }
    }
}

impl From<RecordType> for u16 {
    fn from(record: RecordType) -> Self {
        record as u16
    }
}

pub struct DomainName {
    domain: String,
}

impl DomainName {
    /// Accepts a slice of bytes, starting from the first length octet.
    /// The slice is allowed to exceed the end of the domain name.
    /// If Ok(), the function returns the DomainName and the rest of the slice,
    /// not including the part containing the parsed domain.
    pub fn from(stream: &[u8]) -> Result<(Self, &[u8]), String> {
        let mut domain = String::new();
        let mut stream_iter = stream.iter();

        // Track the length of the domain
        let mut domain_length_octets = 0;
        loop {
            let label_length = *stream_iter.next().ok_or("No length found.")?;

            if label_length == 0 {
                break;
            }

            for _ in 0..label_length {
                let parsed = [*stream_iter.next().ok_or("Couldn't parse octet")?];
                let letter = std::str::from_utf8(&parsed).map_err(|_| "Couldn't parse letter")?;
                domain.push_str(letter);

                domain_length_octets += 1;
            }
            domain.push_str(".");
            domain_length_octets += 1;
        }
        // By this point we should have the domain

        Ok((
            Self {domain},
            &stream[domain_length_octets + 1..]
        ))
    }
}
