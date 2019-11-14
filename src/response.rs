use crate::router;
use std::collections::HashMap;

pub fn response_for_request(path: String, url: String, handler: HashMap<String, HashMap<String, fn()>>) {
    println!("{}", path);
}