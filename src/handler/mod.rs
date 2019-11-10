use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::fs;

mod parser;

pub fn validate_http(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    parser::parse_request(&buffer);
}

