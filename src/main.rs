use rs_webserver::*;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use std::fs;

fn main() {
    let config = fs::read_to_string("config/config.json")
        .expect("Failed to read config file!");
    
    let config: serde_json::Value = serde_json::from_str(config.as_str())
        .expect("Failed to parse config file!");

    let addr = config["address"].as_str().unwrap();
    println!("Starting server at {}!", &addr);
    let listener = TcpListener::bind(&addr)
        // .expect(format!("Failed to bind to {}!", addr).as_str());
        .unwrap_or_else(|_| panic!("Failed to bind to {}!", addr));
    let pool = ThreadPool::new(thread::available_parallelism().unwrap().get())
        .expect("Failed to create thread pool!");

    let mut tmp_endpoints = vec![];
    if let Some(endpoint) = config.get("endpoints") {
        for endpt in endpoint.as_array().unwrap() {
            tmp_endpoints.push(Endpoint::new(
                endpt.get("uri").unwrap().as_str().unwrap(),
                endpt.get("res").unwrap().as_str().unwrap(),
                endpt.get("doc").unwrap().as_str().unwrap()
            ));
        }
    }

    // TODO: `config` goes out of scope without this `unsafe` block for some reason, fix that sh-

    #[allow(unused_assignments)]
    let mut endpoints: Vec<Endpoint> = vec![];
    unsafe {
        endpoints = std::mem::transmute_copy(&tmp_endpoints);
    }
    let endpoints = Arc::new(endpoints);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        let endpoints = Arc::clone(&endpoints);
        pool.execute(move || {
            // handle_connection(stream);
            handle_connection(stream, endpoints);
        });
    }
}
