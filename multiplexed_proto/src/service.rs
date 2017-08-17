extern crate futures;
extern crate tokio_service;

use self::futures::future;
use self::futures::future::{Future, BoxFuture};
use self::tokio_service::{Service};
use std::io;

pub struct Echo;

impl Service for Echo {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call (&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

