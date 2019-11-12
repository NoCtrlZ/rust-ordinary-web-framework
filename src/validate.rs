// extern crate regex;

// use regex::Regex;

struct Request {
    header: Box<Header>,
    body: Box<Body>,
}

struct Header {
    method: String,
    path: String,
    host: String,
    content_type: String,
}

struct Body {
    json: String,
}

pub fn parse_request(buffer: &[u8]) {
    let request = String::from_utf8_lossy(&buffer[..]);
    let v: Vec<&str> = request.split("\r\n\r\n").collect();
    parse_header(v[0]);
    // println!("Request: {:?}", v[0]);
}

fn parse_header(header: &str) {
    let components: Vec<&str> = header.split("\r\n").collect();
    println!("{:?}", components);
    if !valid_request(components[0]) {
        println!("this request is invalid");
    }
    let request: Vec<&str> = components[0].split(" ").collect();
    let mut header = Header {
        method: "".to_string(),
        path: "".to_string(),
        host: "".to_string(),
        content_type: "".to_string(),
    };
}

fn valid_request(request: &str) -> bool {
    let v: Vec<&str> = request.split(" ").collect();
    if valid_method(v[0]) && valid_path(v[1]) && valid_version(v[2]) {
        return true
    }

    false
}

fn valid_method(method: &str) -> bool {
    match method.as_ref() {
        "GET" | "POST" | "PUT" | "DELETE" | "TRACE" | "CONNECT" |"HEAD" | "OPTIONS" => return true,
        _ => return false,
    };
}

fn valid_path(path: &str) -> bool {
    println!("{}", path);
    if !path.starts_with("/") {
        println!("invalid request: path is not valid");
        return false
    }
    //  Todo let path_checker = Regex::new(r"^(?=.*a).*$").unwrap();
    true
}

fn valid_version(version: &str) -> bool {
    if version == "HTTP/1.1" {
        return true
    }

    false
}
