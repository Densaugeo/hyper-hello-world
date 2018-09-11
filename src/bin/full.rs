extern crate hyper;
extern crate futures;

use hyper::{Body, Request, Response, Server};
use hyper::rt::Future;
use hyper::server::Builder;
use hyper::server::conn::AddrIncoming;

static TEXT: &str = "Hello world!\n";

fn handle_error(e: hyper::Error) -> () {
    eprintln!("Server error: {}", e)
}

// Now all of the closures - and most of the automatic type inferences - are removed:

// This is a Service implementation. A service is created for each connection
pub struct HelloService;
impl hyper::service::Service for HelloService {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = std::io::Error;
    type Future = Box<Future<Item = Response<Self::ResBody>, Error = Self::Error> + Send>;

    fn call(&mut self, _req: Request<Self::ReqBody>) -> Self::Future {
        // This is where the code from handle_request() went
        // future::ok creates a Future that immediately resolves (similar to Result::Ok)
        // Wrap in a Box (managed pointer), because Box has a known size
        Box::new(futures::future::ok(Response::new(Body::from(TEXT))))
    }
}

// This is a NewService implementation. The Server uses a NewService to create new Services
pub struct ServiceMaker;
impl hyper::service::NewService for ServiceMaker {
    type ReqBody = Body;
    type ResBody = Body;
    type Error = std::io::Error;
    type InitError = std::io::Error;
    type Service = HelloService;
    type Future = Box<Future<Item = Self::Service, Error = Self::InitError> + Send>;
    
    fn new_service(&self) -> Self::Future {
        Box::new(futures::future::ok(HelloService{}))
    }
}

// Full type annotations added. Look at all the type parameters!
fn main() {
    let addr: std::net::SocketAddr = ([127, 0, 0, 1], 3000).into();
    
    let service_maker: ServiceMaker = ServiceMaker{};
    
    let builder: Builder<AddrIncoming> = Server::bind(&addr);
    let server_no_error_handling: Server<AddrIncoming, ServiceMaker> = builder.serve(service_maker);
    let server: futures::future::MapErr<Server<AddrIncoming, ServiceMaker>, fn(hyper::Error) -> ()> = server_no_error_handling.map_err(handle_error);

    println!("Starting server at {}...", addr);
    hyper::rt::run(server);
}
