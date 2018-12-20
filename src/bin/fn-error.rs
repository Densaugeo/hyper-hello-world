use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn;

static TEXT: &str = "Hello World!\n";

// This request handler returns a Result which could have an Error (except it always returns Ok in this 
// example)
fn handle_request_can_error(_req: Request<Body>) -> Result<Response<Body>, std::io::Error> {
    Ok(Response::new(Body::from(TEXT)))
}

fn handle_error(e: hyper::Error) -> () {
    eprintln!("Server error: {}", e)
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    
    let service_maker = || {
        // Use service_fn() instead of service_fn_ok() for handlers that can error
        service_fn(handle_request_can_error)
    };

    let builder = Server::bind(&addr);
    let server_no_error_handling = builder.serve(service_maker);
    let server = server_no_error_handling.map_err(handle_error);
    
    println!("Starting server at {}...", addr);
    hyper::rt::run(server);
}
