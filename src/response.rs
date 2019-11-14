use std::collections::HashMap;

pub fn response_for_request(method: String, url: String, handler: HashMap<String, HashMap<String, fn()>>) {
    let func = handler.get(&method).and_then(|m| m.get(&url)).unwrap();
    // println!("{:?}", func);
    func();

    let content = response_content(url);
}

pub fn response_content(url: String) -> String {
    "helo".to_string()
}