pub fn parse_request(buffer: &[u8]) {
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}