use std::net::TcpListener;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::prelude::*;

use crate::request;
use crate::response;
use crate::router;

pub struct Server {
    router: router::Router,
}

impl Server {
    pub fn new(router: router::Router) -> Server {
        Server{ router: router }
    }

    pub fn start(&self, addr: &str) {
        let listener = TcpListener::bind(addr).unwrap();
        for stream in listener.incoming() {
            self.handle(&mut stream.unwrap());
        }
    }

    fn handle(&self, stream: &mut TcpStream) {
        let req = request::Request::parse(stream);
        // self.response(stream, self.handler)
    }

    // fn response(stream: &mut TcpStream, handler: Handler, req: Request) {

    // }
}
