mod parser;
mod router;
mod response;
mod server;

// fn get_dir() -> std::io::Result<PathBuf> {
//     let path = env::current_dir()?;
//     Ok(path)
// }

fn index_action() {
    println!("hello world");
}

fn main() {
    let port = ":5000";
    let mut router = router::register_get("/", index_action);
    server::instance_listen(port, router);
}