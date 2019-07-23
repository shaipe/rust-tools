extern crate hyper;

use hyper::{Method, Request, Body, header::HeaderValue};

fn main() {
    let json = r#"{"library":"hyper"}"#;
    let uri: hyper::Uri = "http://httpbin.org/post".parse().unwrap();
    let mut req = Request::new(Body::from(json));
    *req.method_mut() = Method::POST;
    *req.uri_mut() = uri.clone();
    let x = req.headers_mut().insert(
        hyper::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json")
    );

    println!("{:?}", x);
}
