use std::net::TcpStream;
use std::io::prelude::*;
use std::collections::HashMap;

struct Request {
    prefix: Box<Prefix>,
    header: Box<Header>,
    body: String,
}

struct Prefix {
    method: String,
    url: String,
    proto: String,
}

struct Header {
    header: HashMap<String, Vec<String>>,
}

pub fn parse_request(stream: TcpStream) {
    let trimed_request = trim_request(stream);
    let request = arrange_request(trimed_request);
    println!("{:?}", request.prefix.method);
    // println!("{:?}", request.method);
}

fn trim_request(mut stream: TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    request.trim_matches(char::from(0)).to_string()
}

fn arrange_request(mut request: String) -> Request {
    // println!("request start");
    // println!("{}", request);
    // println!("request end");
    let (head, body) = divide_request(request);
    let (prefix, header) = create_header(head);
    // println!("{:?}", prefix);
    // println!("{:?}", header);
    // println!("{:?}", header.header);
    let req = Request {
        prefix: Box::new(prefix),
        header: Box::new(header),
        body: "String".to_string(),
    };
    req
}

fn divide_request(mut request: String) -> (String, String) {
    let v: Vec<&str> = request.split("\r\n\r\n").collect();
    (v[0].to_string(), v[1].to_string())
}

fn create_header(head: String) -> (Prefix, Header) {
    let pre: Vec<&str> = head.split("\r\n").collect();
    let (prefix, header) = set_header(pre);
    // let prefix = set_prefix(pre[0].to_string());
    // let header = set_header(pre[1].to_string());

    (prefix, header)
}

fn set_header(headers: Vec<&str>) -> (Prefix, Header) {
        let mut header = Header {
            header: HashMap::new(),
        };
        let v: Vec<&str> = headers[0].split(" ").collect();
        let prefix = Prefix {
            method: v[0].to_string(),
            url: v[1].to_string(),
            proto: v[2].to_string(),
        };

        for n in 1..headers.len() {
            let mut v: Vec<&str> = headers[n].split(" ").collect();
            let mut pre = v[0].to_string();
            pre.pop();
            // println!("{}", pre);
            // println!("{}", v[1]);
            header.header.insert(
                pre,
                vec![v[1].to_string()],
            );
        };
        // println!("{}", prefix.method);
        // println!("{:?}", header.header);
        (prefix, header)
}
