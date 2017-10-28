pub extern crate hyper;
pub extern crate futures;
extern crate reqwest;
extern crate tokio_service;

use std::thread;

use hyper::server::{Http, NewService};
use hyper::{Error, Response, Request, Body};
use futures::{Stream, Future};

mod service_fn;
pub use service_fn::{service_fn, ServiceFn};

pub fn serve_str<S>(s: S) -> Server
    where S: ToString
{
    let s = s.to_string();
    Server::run(move || {
        let s = s.clone();
        Ok(service_fn(move |_req| Ok(Response::<Body>::new().with_body(s.clone()))))
    })
}

pub fn serve<B, S, Bd>(handler: S) -> Server
    where
        S: NewService<Request = Request, Response = Response<Bd>, Error = Error> + Send + Sync + 'static,
        Bd: Stream<Item = B, Error = Error> + 'static,
        B: AsRef<[u8]> + 'static,
{
    Server::run(handler)
}

/// A HTTP server listening on a loopback interface on a system chosen port designed for use in
/// unit and end to end tests. The server runs in a background thread and is cleanly stopped and
/// shutdown when the Server intance is dropped. Any errors result in a panic either in the thread
/// or on the caller thread.
pub struct Server {
    child: Option<thread::JoinHandle<()>>,
    done: Option<futures::sync::oneshot::Sender<()>>,
    local_addr: std::net::SocketAddr,
}

impl Server {
    /// Creates and runs a http server with the given handler in a background thread. The server is
    /// stopped and the thread cleaned up once the returned instance of Server is dropped.
    pub fn run<B, S, Bd>(handler: S) -> Server
    where
        S: NewService<Request = Request, Response = Response<Bd>, Error = Error> + Send + Sync + 'static,
        Bd: Stream<Item = B, Error = Error> + 'static,
        B: AsRef<[u8]> + 'static,
    {
        let (done_tx, done_rx) = futures::sync::oneshot::channel();
        let (addr_tx, addr_rx) = std::sync::mpsc::channel();

        let child = Some(thread::spawn(move || {
            let server = Http::new()
                .bind(&([127, 0, 0, 1], 0).into(), handler)
                .unwrap();
            addr_tx.send(server.local_addr().unwrap()).unwrap();
            server.run_until(done_rx.map_err(|_| ())).unwrap();
        }));

        Server {
            child: child,
            done: Some(done_tx),
            local_addr: addr_rx.recv().unwrap(),
        }
    }

    /// Get the url that the server is running on.
    pub fn url(&self) -> String {
        format!("http://{}", self.local_addr)
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        if let Some(done) = self.done.take() {
            done.send(()).unwrap();
        }
        if let Some(child) = self.child.take() {
            child.join().unwrap();
        }
    }
}
