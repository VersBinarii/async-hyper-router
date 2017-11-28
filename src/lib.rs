#![doc(html_root_url = "")]

//! # Async Hyper Router
//!
//! Library providing a very basic router functionality for the Async Hyper lib
//!
//! ## Usage
//!
//! To use the library just add: 
//! 
//! ```text
//! async-hyper-router = { git = "https://github.com/username/project-name.git" }
//! ```
//!
//! to your dependencies.
//!
//! ```no_run
//!#[macro_use]
//!extern crate lazy_static;
//!
//!extern crate tokio_core;
//!extern crate hyper;
//!extern crate futures;
//!extern crate hyper_router;
//!
//!use futures::future::FutureResult;
//!
//!use hyper::StatusCode;
//!use hyper::header::ContentLength;
//!use hyper::server::{Http, Request, Response, Service};
//!use hyper_router::{HyperRouter};
//!
//!static HELLO_GET: &'static [u8] = b"Hello, Get!";
//!static HELLO_POST: &'static [u8] = b"Hello, Post!";
//!
//!lazy_static! {
//!    static ref ROUTER: HyperRouter = HyperRouter::new()
//!        .get("/hello_get", get_handler)
//!        .post("/hello_post", post_handler);
//!}
//!
//!fn get_handler(_: Request) -> Response {
//!    Response::new()
//!        .with_header(ContentLength(HELLO_GET.len() as u64))
//!        .with_body(HELLO_GET)
//!}
//!
//!fn post_handler(_: Request) -> Response {
//!    Response::new()
//!        .with_header(ContentLength(HELLO_POST.len() as u64))
//!        .with_body(HELLO_POST)
//!}
//!
//!struct MyServer(&'static HyperRouter);
//!
//!impl Service for MyServer {
//!    type Request = Request;
//!    type Response = Response;
//!    type Error = hyper::Error;
//!    type Future = FutureResult<Response, hyper::Error>;
//!
//!    fn call(&self, req: Request) -> Self::Future {
//!        
//!        futures::future::ok(match self.0.find_handler(&req) {
//!            Ok(handler) => handler(req),
//!            Err(_) => Response::new()
//!                .with_status(StatusCode::NotFound),
//!        })
//!    }
//!}
//!
//!fn main() {
//!    let addr = "127.0.0.1:3000".parse().unwrap();
//!    
//!    let mut server = Http::new().bind(&addr, || Ok(MyServer(&ROUTER))).unwrap();
//!
//!    server.no_proto();
//!    println!("Listening on http://{}.", server.local_addr().unwrap());
//!    server.run().unwrap();
//!}
//! ```


extern crate hyper;
extern crate regex;

use regex::Regex; 
use hyper::server::{Request, Response, };
use hyper::{Method, StatusCode};

struct Path(Regex);

impl Path {
    pub fn new(path: &str) -> Self {
        Path(Regex::new(path).unwrap())
    }
}

type RouteHandler = fn(Request) -> Response;

struct Route {
    method: Method,
    path: Path,
    handler: RouteHandler
}

pub struct HyperRouter {
    routes: Vec<Route>
}

impl HyperRouter {
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }
    pub fn get(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.push(Route {
            method: Method::Get,
            path: Path::new(path),
            handler: handler
        });
        self
    }
    pub fn post(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.push(Route {
            method: Method::Post,
            path: Path::new(path),
            handler: handler
        });
        self
    }
    pub fn put(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.push(Route {
            method: Method::Put,
            path: Path::new(path),
            handler: handler
        });
        self
    }
    pub fn patch(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.push(Route {
            method: Method::Patch,
            path: Path::new(path),
            handler: handler
        });
        self
    }
    pub fn delete(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.push(Route {
            method: Method::Delete,
            path: Path::new(path),
            handler: handler
        });
        self
    }
    pub fn options(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.push(Route {
            method: Method::Options,
            path: Path::new(path),
            handler: handler
        });
        self
    }
    pub fn head(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.push(Route {
            method: Method::Head,
            path: Path::new(path),
            handler: handler
        });
        self
    }
    pub fn trace(mut self, path: &str, handler: RouteHandler) -> Self {
        self.routes.push(Route {
            method: Method::Trace,
            path: Path::new(path),
            handler: handler
        });
        self
    }

    pub fn find_handler(&self, method: &Method, path: &str) -> Result<RouteHandler, StatusCode> {
        let methods = self.routes.iter()
            .filter(|r| r.method == *method).collect::<Vec<_>>();

        if methods.len() == 0 {
            return Err(StatusCode::NotImplemented)
        }
        methods.iter()
            .find(|r| r.path.0.is_match(path))
            .map(|r| Ok(r.handler))
            .unwrap_or(Err(StatusCode::NotFound))
    }
}
