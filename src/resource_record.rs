pub enum ResourceRecordTypes {
    A,
    NS,
    CNAME,
    SOA,
    WKS,
    PTR,
    HINFO,
    MINFO,
    MX,
    TXT
}

pub enum ResourceClassTypes {
    IN,
    CN,
    HS
}

pub struct ResourceRecord {
    name: String,
    rtype: ResourceRecordTypes,
    class: ResourceClassTypes,
    ttl: u32,
    rd_length: u16,
    r_data: String
}