extern crate reqwest;
extern crate httptest;

use std::io::Read;

fn main() {
    let ts = {
        httptest::Server::new().run()
    };

    println!("ts url: {}", ts.url());
    let mut response = reqwest::get(ts.url()).unwrap();
    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();

    println!("{}", body);

    assert_eq!(body, "hello world!");
}
