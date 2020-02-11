mod request;
mod response;
mod router;
mod server;
use std::collections::HashMap;

fn main() {
    let mut router = router::Router::new();
    router.get("/", index_handler);
    server::Server::new(router).start("127.0.0.1:5000")
}

fn index_handler(req: request::Request) -> response::Response {
    let mut book_reviews = HashMap::new();
    book_reviews.insert(
        "Adventures of Huckleberry Finn".to_string(),
        "My favorite book.".to_string(),
    );
    // println!("{:?}", req);
    response::Response {
        prefix: response::prefix::PREFIX.to_string(),
        header: book_reviews,
        body: "Test".to_string(),
    }
}
