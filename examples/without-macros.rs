extern crate hyper;
extern crate reqwest;
extern crate httptest;

use std::io::Read;

fn main() {
    let ts = httptest::Server::run(|| {
        Ok(httptest::service_fn(|_req| {
            Ok(httptest::Response::<hyper::Body>::new().with_body(
                "hello world",
            ))
        }))
    });

    let mut response = reqwest::get(&ts.url()).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    assert_eq!(body, "hello world");
}
