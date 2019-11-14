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
    // println!("{:?}", request.method);
}

fn trim_request(mut stream: TcpStream) -> String {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    request.trim_matches(char::from(0)).to_string()
}

fn arrange_request(mut request: String) -> Request {
    println!("request start");
    println!("{}", request);
    println!("request end");
    let (head, body) = divide_request(request);
    let (prefix, header) = create_header(head);
    // println!("{:?}", prefix);
    // println!("{:?}", header);
    println!("{:?}", header.header);
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

fn set_header(header: Vec<&str>) -> (Prefix, Header) {
        // if n == 0 {
        //     println!(header[n])
        // }
        let prefix = Prefix {
            method: "hi".to_string(),
            url: "hi".to_string(),
            proto: "hi".to_string(),
        };

        let mut map = Header {
            header: HashMap::new(),
        };

        map.header.insert(
            "Adventures of Huckleberry Finn".to_string(),
            vec!["My favorite book.".to_string(), "hello".to_string()],
        );
        println!("{:?}", prefix.method);
        (prefix, map)
}

fn set_prefix(pre: String) -> Prefix {
    let v: Vec<&str> = pre.split(" ").collect();
    let prefix = Prefix {
        method: v[0].to_string(),
        url: v[1].to_string(),
        proto: v[2].to_string(),
    };
    prefix
}
