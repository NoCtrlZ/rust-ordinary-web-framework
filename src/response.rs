use std::collections::HashMap;
use std::path::PathBuf;
use std::env;
use std::fs;

pub fn response_for_request(method: String, url: String, handler: HashMap<String, HashMap<String, fn()>>) -> String {
    let func = handler.get(&method).and_then(|m| m.get(&url)).unwrap();
    // println!("{:?}", func);
    func();

    response_content(url)
}

pub fn response_content(mut url: String) -> String {
    let mut path = get_view_dir().unwrap();
    // println!("{:?}", path);
    url.pop();
    if url != "".to_string() {
        path.push(url);
    } else {
        path.push(r"index")
    }
    path.set_extension("ers");
    println!("{:?}", path);
    let contents = fs::read_to_string(path).unwrap();
    format!("HTTP/1.1 200 OK\r\n\r\n{}", contents)
}

fn get_view_dir() -> std::io::Result<PathBuf> {
    let mut path = env::current_dir()?;
    path.push(r"templates");
    Ok(path)
}