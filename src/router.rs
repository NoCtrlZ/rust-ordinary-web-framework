pub type Handler = fn(Request) -> Response;

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
            handler: Handler
        };
        self.routes.push(route)
    }
}