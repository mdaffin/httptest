extern crate hyper;
extern crate futures;
extern crate service_fn;
extern crate reqwest;

use hyper::header::{ContentLength, ContentType};
use hyper::server::{Http, Response};
use futures::future;
use std::thread;
use service_fn::service_fn;
use std::sync::{Mutex, Arc};

static TEXT: &'static str = "Hello, World!";

pub struct Server {
    close: Arc<Mutex<bool>>,
}
pub struct ServerBuilder {}

impl ServerBuilder {
    pub fn run(self) -> Server {
        let handler = || {
            Ok(service_fn(|_req| {
                println!("get a request");
                Ok(
                    Response::<hyper::Body>::new()
                        .with_header(ContentLength(TEXT.len() as u64))
                        .with_header(ContentType::plaintext())
                        .with_body(TEXT),
                )
            }))
        };

        let addr = ([127, 0, 0, 1], 3000).into();



        let close = Arc::new(Mutex::new(true));
        let close_clone = close.clone();

        println!("starting thread");
        thread::spawn(move || {
            let server = Http::new().bind(&addr, handler).unwrap();
            println!("starting server");
            server.run_until(future::poll_fn(|| if *close_clone.lock().unwrap() {
                Ok(futures::Async::NotReady)
            } else {
                Ok(futures::Async::Ready(()))
            }))
        });
        Server { close }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        *self.close.lock().unwrap() = true;
    }
}

impl Server {
    pub fn new() -> ServerBuilder {
        ServerBuilder {}
    }

    pub fn url(&self) -> &str {
        "http://127.0.0.1:80"
        //self.inner.local_addr()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn simple_json() {}
}
