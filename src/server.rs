use std::net::TcpListener;
use std::collections::HashMap;

use crate::parser;
use crate::response;

pub fn instance_listen(port: &str, handler: HashMap<String, HashMap<String, fn()>>) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();
    // println!("{:?}", handler);

    for stream in listener.incoming() {

        let handler = handler.clone();
        let stream = stream.unwrap();

        let request = parser::parse_request(stream);
        // println!("{:?}", request.body);
        response::response_for_request(request.prefix.method, request.prefix.url, handler);
    }
}