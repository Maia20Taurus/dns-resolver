use std::net::UdpSocket;
use dns_resolver::message::header::Header;
use dns_resolver::message::*;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 512];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let filled_buf = &mut buf[..amt];

        let mut header = Header::from(filled_buf).unwrap();
        println!("Header: {:?}", header);

        // send back to origin
        socket.send_to(&[0; 512], &src)?;

        let mut qname_iter = filled_buf[12..].iter();
        let mut domain = String::new();

        loop {
            let label_length = *qname_iter.next().expect("No length found.");

            if label_length == 0 {
                break;
            }

            for _ in 0..label_length {
                let parsed = [*qname_iter.next().expect("Couldn't parse octet")];
                let letter = std::str::from_utf8(&parsed).expect("Couldn't parse letter");
                domain.push_str(letter);
            }
            domain.push_str(".");
        }

        println!("Parsed domain: {}", domain);

        let x = filled_buf[3];
        // check all the bits from LSB to MSB
        for i in 0..8 {
            let mask = 1 << i;
            let bit_is_set = (mask & x) > 0;
            println!("bit number {i} is set: {bit_is_set}");
        }
    } // the socket is closed here
    Ok(())
}
