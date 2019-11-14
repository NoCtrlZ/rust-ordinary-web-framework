mod parser;
mod router;
mod response;
mod server;
mod controller;

fn main() {
    let port = ":5000";
    let mut router = router::register_get("/", controller::index);
    server::instance_listen(port, router);
}