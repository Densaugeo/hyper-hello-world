extern crate hyper;

use hyper::{Body, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

static TEXT: &str = "Hello world!\n";

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    
    // service_maker is called for every new connection, to spawn a new Service
    let service_maker = || {
        // service_fn_ok handles Hyper's bookkeeping for creating a new Service from a function
        service_fn_ok(|_req| {
            Response::new(Body::from(TEXT))
        })
    };

    // Server::bind() creates a Builder. The Builder has several methods for configuring options, and a .serve() method for
    // creating a Server.
    let server = Server::bind(&addr)
        // .serve() creates a Server, which runs service_maker for each new connection
        .serve(service_maker)
        // The Futures way of adding error handling
        .map_err(|e| eprintln!("Server error: {}", e));
    
    println!("Starting server at {}...", addr);
    hyper::rt::run(server);
}
