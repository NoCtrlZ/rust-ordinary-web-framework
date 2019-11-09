use std::net::TcpListener;

fn instance_listen(port: &str) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("TCP Connection");
    }
}

fn main() {
    let port = ":5000";
    instance_listen(port);
}