use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpStream;

pub mod method {
    pub const GET: &str = "GET";
    pub const POST: &str = "POST";
}

pub struct Request {
    path: String,
    method: String,
    params: HashMap<String, String>,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) {
        let raw_data = convert(stream);
        let (prefix, header, body) = divide(&raw_data);
        println!("{:?}", prefix);
        // reader.read(&mut lines);
        // let contents = String::from_utf8_lossy(&buffer[..]);
    }

}

fn convert(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer[..]).trim_matches(char::from(0)).to_string()
}

fn divide(raw_data: &str) -> (String, String, String) {
    let components: Vec<&str> = raw_data.split("\r\n\r\n").collect();
    let (prefix, header) = divide_none_body(components[0]);
    (prefix, header, components[1].to_string())
}

fn divide_none_body(none_body: &str) -> (String, String) {
    let components: Vec<&str> = none_body.split("\r\n").collect();
    (components[0].to_string(), components[1].to_string())
}
