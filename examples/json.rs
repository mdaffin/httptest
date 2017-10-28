#[macro_use]
extern crate serde_derive;
extern crate hyper;
extern crate reqwest;
extern crate httptest;

use std::io::Read;

fn main() {
    #[derive(Serialize)]
    struct data<'a> {
        key: &'a str,
        value: &'a str,
    }

    let ts = httptest::serve_json(data {
        key: "hello",
        value: "world!",
    });

    let mut response = reqwest::get(&ts.url()).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    assert_eq!(body, "{\"key\":\"hello\",\"value\":\"world!\"}");
}
