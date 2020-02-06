use crate::request;
use crate::response;

pub mod Method {
    pub const GET: &str = "GET";
    pub const POST: &str = "POST";
}

pub type Handler = fn(request::Request) -> response::Response;

pub struct Route {
    pub path: String,
    pub method: String,
    pub handler: Handler,
}

pub struct Router {
    pub routes: Vec<Route>,
}

impl Router {
    pub fn new() -> Router {
        Router { routes: Vec::new() }
    }

    pub fn register(&mut self, method: &str, path: &str, handler: Handler) {
        let route = Route {
            method: String::from(method),
            path: String::from(path),
            handler: handler,
        };
        self.routes.push(route)
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        const method: &str = Method::GET;
        self.register(method, path, handler);
    }

    pub fn post(&mut self, path: &str, handler: Handler) {
        const method: &str = Method::POST;
        self.register(method, path, handler);
    }
}
