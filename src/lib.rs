mod threading;
pub use threading::*;

use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::Arc;

#[derive(Debug)]
pub struct Endpoint<'a> {
    uri: &'a str,
    res: &'a str,
    doc: &'a str,
}

impl<'a> Endpoint<'a> {
    pub fn new(uri: &'a str, res: &'a str, doc: &'a str) -> Endpoint<'a> {
        Endpoint { uri, res, doc }
    }
}

pub fn handle_connection(mut stream: TcpStream, endpoints: Arc<Vec<Endpoint>>) {
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();

    let mut buffer = String::new();
    stream.read_to_string(&mut buffer).unwrap();

    let (mut status_line, mut document) = (
        "HTTP/1.1 404 NOT FOUND".to_string(),
        "res/404.html".to_string(),
    );
    for endpoint in endpoints.iter() {
        if buffer.starts_with(format!("GET {} HTTP/1.1\r\n", endpoint.uri).as_str()) {
            status_line = format!("HTTP/1.1 {}", endpoint.res);
            document = endpoint.doc.to_string();
        }
    }

    let contents = fs::read_to_string(&document)
        // .expect(format!("Failed to retrieve document: {}", document).as_str());
        .unwrap_or_else(|_| panic!("Failed to retrieve document: {}!", document));
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
