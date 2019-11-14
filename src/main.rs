mod parser;
mod router;
mod response;
mod server;
mod controller;

// fn get_dir() -> std::io::Result<PathBuf> {
//     let path = env::current_dir()?;
//     Ok(path)
// }

fn main() {
    let port = ":5000";
    let mut router = router::register_get("/", controller::index);
    server::instance_listen(port, router);
}