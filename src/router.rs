use std::collections::HashMap;

pub struct Router {
    pub handler: HashMap<String, HashMap<String, fn()>>,
}

pub fn register_get(path: &str, func: fn()) -> Router {
    let mut router = Router {
		handler: HashMap::new(),
	};
    router.handler.entry("GET".to_string()).or_insert_with(HashMap::new).insert(path.to_string(), func);
    println!("{:?}", router.handler["GET"][path]);
    router
}