use std::net::TcpListener;
use std::collections::HashMap;

mod parser;
mod router;

// fn get_dir() -> std::io::Result<PathBuf> {
//     let path = env::current_dir()?;
//     Ok(path)
// }

fn index_action() {
    println!("hello world");
}

pub fn register_get(path: &str, func: fn()) {
    let mut route = HashMap::new();
    route.entry("GET").or_insert_with(HashMap::new).insert(path, func);
    println!("{:?}", route["GET"][path]);
}

fn instance_listen(port: &str) {
    let listener = TcpListener::bind(format!("localhost{}", port)).unwrap();

    for stream in listener.incoming() {

        let stream = stream.unwrap();

        let request = parser::parse_request(stream);
        println!("{:?}", request.body);

        // map.entry("GET").or_insert_with(HashMap::new).insert("/", index_action);
    }
}

fn main() {
    let port = ":5000";
    let mut route = register_get("/", index_action);
    instance_listen(port);
}