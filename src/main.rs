use std::net::TcpListener;
use std::thread;
use std::sync::Arc;
use rs_webserver::*;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878")
        .expect("Failed to bind to localhost:7878!");
    let pool = ThreadPool::new(thread::available_parallelism().unwrap().get())
        .expect("Failed to create thread pool!");

    let endpoints: Arc<Vec<Endpoint>> = Arc::new(vec![
        Endpoint::new("/", "200 OK", "res/default.html"),
        Endpoint::new("/sleep", "200 OK", "res/default.html"),
        Endpoint::new("/shutdown", "200 OK", "res/default.html")
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