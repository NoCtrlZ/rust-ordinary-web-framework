use std::net::TcpStream;
use std::io::prelude::*;

pub fn parse_request(stream: TcpStream) {
    let request = trim_request(stream);
    println!("{:?}", request);
}

fn trim_request(mut stream: TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    request.trim_matches(char::from(0)).to_string()
}