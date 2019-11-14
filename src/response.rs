use crate::router;
use std::collections::HashMap;

pub fn response_for_request(path: String, url: String, handler: HashMap<String, HashMap<String, fn()>>) {
    let func = handler.get(&path).and_then(|m| m.get(&url)).unwrap();
    println!("{:?}", func);
    func();
}