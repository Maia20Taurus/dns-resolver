pub mod header;
pub mod question;

#[derive(Debug)]
pub enum Qr {
    Query,
    Response
}

impl Qr {
    fn value(&self) -> u16 {
        match self {
            Qr::Query => 0,
            Qr::Response => 1
        }
    }
}

#[derive(Debug)]
pub enum Opcode {
    Query,
    IQuery,
    Status
}

impl Opcode {
    fn value(&self) -> u16 {
        match self {
            Opcode::Query => 0,
            Opcode::IQuery => 1,
            Opcode::Status => 2
        }
    }
}

#[derive(Debug)]
pub enum ResponseCode {
    NoError,
    FormatError,
    ServerFailure,
    NameError,
    NotImplemented,
    Refused
}

impl ResponseCode {
    fn value(&self) -> u16 {
        match self {
            ResponseCode::NoError => 0,
            ResponseCode::FormatError => 1,
            ResponseCode::ServerFailure => 2,
            ResponseCode::NameError => 3,
            ResponseCode::NotImplemented => 4,
            ResponseCode::Refused => 5
        }
    }
}