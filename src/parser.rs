use std::net::TcpStream;
use std::io::prelude::*;

struct Request {
    prefix: Box<Prefix>,
    header: String,
    body: String,
}

struct Prefix {
    method: String,
    url: String,
    proto: String,
}

pub fn parse_request(stream: TcpStream) {
    let trimed_request = trim_request(stream);
    let request = arrange_request(trimed_request);
    // println!("{:?}", request.method);
}

fn trim_request(mut stream: TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    request.trim_matches(char::from(0)).to_string()
}

fn arrange_request(mut request: String) -> Request {
    let (head, body) = divide_request(request);
    let prefix = set_prefix(head);
    // println!("{:?}", prefix);
    // println!("{:?}", header);
    let req = Request {
        prefix: Box::new(prefix),
        header: "String".to_string(),
        body: "String".to_string(),
    };
    req
}

fn divide_request(mut request: String) -> (String, String) {
    let v: Vec<&str> = request.split("\r\n\r\n").collect();
    (v[0].to_string(), v[1].to_string())
}

fn set_prefix(head: String) -> Prefix {
    let pre: Vec<&str> = head.split("\r\n").collect();
    let v: Vec<&str> = pre[0].split(" ").collect();
    let prefix = Prefix {
        method: v[0].to_string(),
        url: v[1].to_string(),
        proto: v[2].to_string(),
    };
    prefix
}
