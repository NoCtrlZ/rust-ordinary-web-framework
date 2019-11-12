use std::net::TcpStream;
use std::io::prelude::*;

pub fn parse_request(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let raw_request = String::from_utf8_lossy(&buffer[..]);
    let request = raw_request.trim_matches(char::from(0));

    println!("{:?}", request);
}