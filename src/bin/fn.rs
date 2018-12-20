use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::service::service_fn_ok;

static TEXT: &str = "Hello world!\n";

// The closures can be rewritten as regular functions
fn handle_request(_req: Request<Body>) -> Response<Body> {
    Response::new(Body::from(TEXT))
}

fn handle_error(e: hyper::Error) -> () {
    eprintln!("Server error: {}", e)
}

fn main() {
    let addr = ([127, 0, 0, 1], 3000).into();
    
    // This last closure is more complex: it must be replaced with NewService implementation
    let service_maker = || {
        // service_fn_ok() can accept a function that turns a Request into a Response
        service_fn_ok(handle_request)
    };

    let builder = Server::bind(&addr);
    let server_no_error_handling = builder.serve(service_maker);
    // .map_err() is a Future method. It expects an error handler to accept an error of some sort and return 
    // unit (Rust's unit is like void in C). In order to compile, every error type the server can generate must 
    // be mapped to unit
    let server = server_no_error_handling.map_err(handle_error);
    
    println!("Starting server at {}...", addr);
    hyper::rt::run(server);
}
