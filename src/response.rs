use std::collections::HashMap;

pub mod prefix {
    pub const PREFIX: &str = "HTTP/1.1 200 OK\r\n";
}

pub struct Response {
    pub prefix: String,
    pub header: HashMap<String, String>,
    pub body: String,
}

impl Response {
    // fn write(&self, stream: &mut TcpStream) -> {
    //     stream.write(response.as_bytes()).unwrap();
    //     stream.flush().unwrap();
    // }

    // fn compile(&self) -> String {
    //     let prefix = prefix::PREFIX;

    // }
}
