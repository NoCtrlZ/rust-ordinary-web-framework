use std::net::TcpStream;
use std::io::prelude::*;

struct Request {
    method: String,
    path: String,
    proto: String,
    host: String,
    header: String,
    body: String,
}

pub fn parse_request(stream: TcpStream) {
    let trimed_request = trim_request(stream);
    let request = arrange_request(trimed_request);
    println!("{:?}", request.method);
}

fn trim_request(mut stream: TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    request.trim_matches(char::from(0)).to_string()
}

fn arrange_request(mut request: String) -> Request {
    let (head, body) = divide_request(request);
    println!("{:?}", head);
    println!("{:?}", body);
    let req = Request {
        method: "String".to_string(),
        path: "String".to_string(),
        proto: "String".to_string(),
        host: "String".to_string(),
        header: "String".to_string(),
        body: "String".to_string(),
    };
    req
}

fn divide_request(mut request: String) -> (String, String) {
    let v: Vec<&str> = request.split("\r\n\r\n").collect();
    (v[0].to_string(), v[1].to_string())
}