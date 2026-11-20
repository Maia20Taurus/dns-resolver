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
        println!("{}",value);
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