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
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}