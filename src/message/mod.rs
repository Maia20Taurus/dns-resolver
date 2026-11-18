pub mod header;
pub mod question;

#[derive(Debug)]
pub enum Qr {
    Query = 0,
    Response = 1
}

#[derive(Debug)]
pub enum Opcode {
    Query = 0,
    IQuery = 1,
    Status = 2
}

#[derive(Debug)]
pub enum ResponseCode {
    NoError = 0,
    FormatError = 1,
    ServerFailure = 2,
    NameError = 3,
    NotImplemented = 4,
    Refused = 5
}