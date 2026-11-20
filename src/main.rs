use dns_resolver::message::header::Header;
use dns_resolver::message::question::Question;
use dns_resolver::message::*;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        // Receives a single datagram message on the socket. If `buf` is too small to hold
        // the message, it will be cut off.
        let mut buf = [0; 512];
        let (amt, src) = socket.recv_from(&mut buf)?;
        let filled_buf = &mut buf[..amt];

        let mut header = Header::from(filled_buf).unwrap();
        let question = Question::from(filled_buf);
        header.set_response(true);
        println!("Header: {:?}", header);
        println!("Question: {:?}", question);

        // send back to origin
        socket.send_to(&header.in_bytes().unwrap(), &src)?;

        let x = filled_buf[3];
        // check all the bits from LSB to MSB
        for i in 0..8 {
            let mask = 1 << i;
            let bit_is_set = (mask & x) > 0;
            //println!("bit number {i} is set: {bit_is_set}");
        }
    } // the socket is closed here
    Ok(())
}
