use std::collections::HashMap;

pub fn register_get(path: &str, func: fn()) -> HashMap<String, HashMap<String, fn()>> {
    let mut router = HashMap::new();
    router.entry("GET".to_string()).or_insert_with(HashMap::new).insert(path.to_string(), func);
    println!("{:?}", router["GET"][path]);
    router
}