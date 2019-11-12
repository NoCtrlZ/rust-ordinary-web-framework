use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;
use std::fs;
use std::env;

mod handler;
mod parser;

// fn get_dir() -> std::io::Result<PathBuf> {
//     let path = env::current_dir()?;
//     Ok(path)
// }

fn instance_listen(port: &str) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();

        parser::parse_request(stream);
    }
}

fn main() {
    let port = ":5000";
    instance_listen(port);
}