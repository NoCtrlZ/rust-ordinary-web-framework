use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::PathBuf;
use std::fs;
use std::env;

fn get_dir() -> std::io::Result<PathBuf> {
    let path = env::current_dir()?;
    Ok(path)
}

fn instance_listen(port: &str) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut dir_path = get_dir().unwrap();
    dir_path.push("template");
    dir_path.push("test");
    dir_path.set_extension("html");

    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string(dir_path).unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let port = ":5000";
    instance_listen(port);
}