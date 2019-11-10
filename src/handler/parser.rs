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
    if valid_request(components[0]) {
        let request: Vec<&str> = components[0].split(" ").collect();
        println!("{:?}", request);
    }
}

fn valid_request(request: &str) -> bool {
    let v: Vec<&str> = request.split(" ").collect();
    valid_method(v[0]);
    // valid_path(v[1]);
    // valid_version(v[2]);
    if v[2] == "HTTP/1.1" {
        true
    } else {
        false
    }
}

fn valid_method(method: &str) -> bool {
    match method.as_ref() {
        "GET" | "POST" | "PUT" | "DELETE" | "TRACE" | "CONNECT" |"HEAD" | "OPTIONS" => return true,
        _ => return false,
    };
}