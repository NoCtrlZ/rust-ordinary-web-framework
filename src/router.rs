use crate::request;
use crate::response;

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
        Router {
            routes: Vec::new(),
        }
    }

    pub fn register(&mut self, method: &str, path: &str, handler: Handler) {
        let route = Route {
            method: String::from(method),
            path: String::from(path),
            handler: handler
        };
        self.routes.push(route)
    }

    pub fn get(&mut self, path: &str, handler: Handler) {
        const method: &str = "GET";
        self.register(method, path, handler);
    }
}
