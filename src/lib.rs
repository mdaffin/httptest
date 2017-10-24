extern crate hyper;
extern crate futures;
extern crate reqwest;
extern crate tokio_service;

use std::thread;

use hyper::server::{Http, Response};
use futures::Future;

mod service_fn;
pub use service_fn::{service_fn, ServiceFn};

pub struct Server {
    child: Option<thread::JoinHandle<hyper::Result<()>>>,
    done: Option<futures::sync::oneshot::Sender<()>>,
    local_addr: std::net::SocketAddr,
}
pub struct ServerBuilder {}

impl ServerBuilder {
    pub fn run(self) -> Server {
        let handler = || {
            Ok(service_fn(|_req| {
                Ok(Response::<hyper::Body>::new().with_body("hello world"))
            }))
        };

        let (done_tx, done_rx) = futures::sync::oneshot::channel();
        let (addr_tx, addr_rx) = std::sync::mpsc::channel();

        let child = Some(thread::spawn(move || {
            let server = Http::new().bind(&([127, 0, 0, 1], 0).into(), handler)?;
            addr_tx.send(server.local_addr().unwrap()).unwrap();
            server.run_until(done_rx.map_err(|_| ()))
        }));

        Server {
            child,
            done: Some(done_tx),
            local_addr: addr_rx.recv().unwrap(),
        }
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        println!("dropping");
        if let Some(done) = self.done.take() {
            done.send(()).unwrap();
        }
        println!("joining");
        if let Some(child) = self.child.take() {
            child.join().unwrap().unwrap();
        }
        println!("done");
    }
}

impl Server {
    pub fn new() -> ServerBuilder {
        ServerBuilder {}
    }

    pub fn url(&self) -> String {
        format!("http://{}", self.local_addr)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_server() {}
}
