mod request;
mod response;
mod router;
mod server;

fn main() {
    let mut router = router::Router::new();
    // router.get("/", index_handler);
    server::Server::new(router).start("127.0.0.1:5000")
}

// fn index_handler(req: request::Request) -> response::Response {
//     println!("{:?}", req);
// }
