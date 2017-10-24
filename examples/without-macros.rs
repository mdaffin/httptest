extern crate reqwest;
extern crate httptest;

use std::io::Read;

fn main() {
    {
        let ts = {
            httptest::Server::new().run()
        };

        println!("ts url: {}", ts.url());
        let mut response = reqwest::get(&ts.url()).unwrap();
        let mut body = String::new();
        response.read_to_string(&mut body).unwrap();

        println!("{}", body);
        println!("sleeping");
        std::thread::sleep(std::time::Duration::from_millis(1000));
        println!("sleeping done");

        assert_eq!(body, "Hello, World!");
    }
    println!("sleeping");
    std::thread::sleep(std::time::Duration::from_millis(5000));
    println!("sleeping done");
}
