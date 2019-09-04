use craft;
use hyper::*;

fn first_handler(_request: &Request<Body>, next: &dyn Fn() -> Response<Body>) -> Response<Body> {
    println!("begin first_handler");
    let response = next();
    println!("end first_handler");
    response
}

fn second_handler(_request: &Request<Body>, next: &dyn Fn() -> Response<Body>) -> Response<Body> {
    println!("begin second_handler");
    let response = next();
    println!("end second_handler");
    response
}

fn hello(_request: &Request<Body>) -> Response<Body> {
    Response::new(Body::from("Hello, World!"))
}

#[runtime::main]
async fn main() {
    let config = craft::Config::new("127.0.0.1", 3000);
    let mut handler_stack = craft::Stack::empty();
    handler_stack.push(first_handler);
    handler_stack.push(second_handler);
    handler_stack.set(hello);
    craft::get("/hello", handler_stack);
    craft::start(&config).await;
}
