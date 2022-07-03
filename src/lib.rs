mod threading;
pub use threading::*;

use std::fs;
use std::io::{prelude::*, BufRead, BufReader};
use std::net::TcpStream;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Endpoint {
    uri: String,
    res: String,
    doc: String,
}

impl Endpoint {
    pub fn new(uri: &str, res: &str, doc: &str) -> Endpoint {
        Endpoint {
            uri: String::from(uri),
            res: String::from(res),
            doc: String::from(doc),
        }
    }
}

pub fn handle_connection(mut stream: TcpStream, endpoints: Arc<Vec<Endpoint>>) {
    let mut buffer = vec![];
    let mut reader = BufReader::new(&stream);
    loop {
        let mut current_read = String::new();
        reader.read_line(&mut current_read).unwrap();
        buffer.append(&mut Vec::from_iter(current_read.as_bytes().iter().copied()));
        if current_read == "\r\n" {
            break;
        }
    }

    println!("buffer: {:?}", String::from_utf8(buffer.clone()));
    println!("buffer len: {}", buffer.len());

    let (mut status_line, mut document) = (
        "HTTP/1.1 404 NOT FOUND".to_string(),
        "res/404.html".to_string(),
    );
    for endpoint in endpoints.iter() {
        if buffer.starts_with(format!("GET {} HTTP/1.1\r\n", endpoint.uri).as_bytes()) {
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
