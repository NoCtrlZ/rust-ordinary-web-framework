use std::net::TcpListener;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::prelude::*;

use crate::parser;
use crate::response;

pub fn instance_listen(port: &str, handler: HashMap<String, HashMap<String, fn()>>) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();
    // println!("{:?}", handler);

    for stream in listener.incoming() {

        let handler = handler.clone();

        let request = parser::parse_request(stream.unwrap());
        // println!("{:?}", request.body);
        let response = response::response_for_request(request.prefix.method, request.prefix.url, handler);
        // println!("{:?}", response);
        // let mut stream = stream.unwrap();

        // stream.write(response.as_bytes()).unwrap();
        // stream.flush().unwrap();
    }
}