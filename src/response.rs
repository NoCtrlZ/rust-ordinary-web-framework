use std::collections::HashMap;
use std::net::TcpStream;

pub mod prefix {
    pub const PREFIX: &str = "HTTP/1.1 200 OK\r\n";
}

pub struct Response {
    pub prefix: String,
    pub header: HashMap<String, String>,
    pub body: String,
}

impl Response {
    pub fn write(&self, stream: &mut TcpStream) {
        let response = &self.body;
        println!("{:?}", response);
    }

    // fn compile(&self) -> String {
    //     let prefix = prefix::PREFIX;

    // }
}
