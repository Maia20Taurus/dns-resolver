pub struct Question {
    // domain name e.g. example.com.
    qname: String,
    // the type of the query, specifying which kind of record is
    // being asked for (e.g. A or MX)
    qtype: u16,
    // the class of the query (IN or CN)
    qclass: u16
}