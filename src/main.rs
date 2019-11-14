use std::net::TcpListener;
use std::collections::HashMap;

mod parser;
mod router;
mod response;

// fn get_dir() -> std::io::Result<PathBuf> {
//     let path = env::current_dir()?;
//     Ok(path)
// }

fn index_action() {
    println!("hello world");
}

fn instance_listen(port: &str) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();

        let request = parser::parse_request(stream);
        println!("{:?}", request.body);
        response::response_for_request(request.prefix.method, request.prefix.url);
    }
}

fn main() {
    let port = ":5000";
    let mut router = router::register_get("/", index_action);
    instance_listen(port);
}