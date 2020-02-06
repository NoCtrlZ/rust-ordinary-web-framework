use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;

use crate::router;

pub struct Request {
    pub method: String,
    pub path: String,
    pub header: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn parse(stream: &mut TcpStream) -> Request {
        let raw_data = convert(stream);
        let (prefix, header, body) = divide(&raw_data);
        let (method, path) = Request::parse_prefix(&prefix);
        let header = Request::parse_header(&header);
        Request {
            method: method,
            path: path,
            header: header,
            body: body,
        }
    }

    pub fn parse_prefix(prefix: &str) -> (String, String) {
        let mut components = prefix.split_whitespace();
        let method = match components.nth(0) {
            Some(e) => e,
            None => router::Method::GET,
        };
        let path = match components.nth(0) {
            Some(e) => e,
            None => "/",
        };
        (method.to_string(), path.to_string())
    }

    pub fn parse_header(header: &str) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        let mut components = header.split_whitespace();
        let item = match components.nth(0) {
            Some(e) => e,
            None => router::Method::GET,
        };
        let content = match components.nth(0) {
            Some(e) => e,
            None => "/",
        };
        headers.insert(item.to_string(), content.to_string());
        headers
    }
}

fn convert(stream: &mut TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    String::from_utf8_lossy(&buffer[..])
        .trim_matches(char::from(0))
        .to_string()
}

fn divide(raw_data: &str) -> (String, String, String) {
    let components: Vec<&str> = raw_data.split("\r\n\r\n").collect();
    if components.len() != 2 {
        panic!("Invalid request data");
    }
    let (prefix, header) = divide_none_body(components[0]);
    (prefix, header, components[1].to_string())
}

fn divide_none_body(none_body: &str) -> (String, String) {
    let components: Vec<&str> = none_body.split("\r\n").collect();
    (components[0].to_string(), components[1].to_string())
}
