use rs_webserver::*;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::env;

fn main() {
    let addr = match env::args().nth(1) {
        Some(arg) => arg,
        None => "127.0.0.1:7878".to_string()
    };
    println!("Starting server at {}!", &addr);
    let listener = TcpListener::bind(&addr).expect(format!("Failed to bind to {}!", addr).as_str());
    let pool = ThreadPool::new(thread::available_parallelism().unwrap().get())
        .expect("Failed to create thread pool!");

    let endpoints: Arc<Vec<Endpoint>> = Arc::new(vec![
        Endpoint::new("/", "200 OK", "res/default.html"),
        Endpoint::new("/sleep", "200 OK", "res/default.html"),
        Endpoint::new("/shutdown", "200 OK", "res/default.html"),
    ]);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let endpoints = Arc::clone(&endpoints);
        pool.execute(move || {
            // handle_connection(stream);
            handle_connection(stream, endpoints);
        });
    }
}
