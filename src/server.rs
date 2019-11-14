use std::net::TcpListener;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::prelude::*;

use crate::parser;
use crate::response;

pub fn instance_listen(port: &str, handler: HashMap<String, HashMap<String, fn()>>) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();

    for stream in listener.incoming() {
        let mut buffer = [0; 512];

        let handler = handler.clone();

        let mut stream = stream.unwrap();

        stream.read(&mut buffer).unwrap();
        let contents = String::from_utf8_lossy(&buffer[..]);
        let contents_string = contents.trim_matches(char::from(0)).to_string();

        let request = parser::parse_request(contents_string);
        let response = response::response_for_request(request.prefix.method, request.prefix.url, handler);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
