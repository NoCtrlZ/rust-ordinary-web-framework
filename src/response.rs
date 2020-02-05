use std::collections::HashMap;

pub mod prefix {
    pub const PREFIX: &str = "HTTP/1.1 200 OK\r\n";
}

pub struct Header {
    pub header: HashMap<String, String>
}

pub struct Response {
    prefix: String,
    header: Vec<Header>,
    body: String,
}
