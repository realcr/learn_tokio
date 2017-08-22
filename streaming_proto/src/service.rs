extern crate tokio_service;
extern crate tokio_proto;
extern crate futures;

use std::io;
use self::futures::{future, Future, BoxFuture, Stream};
use self::tokio_proto::streaming::{Message, Body};
use self::tokio_service::Service;

pub struct PrintStdout;

impl Service for PrintStdout {
    type Request = Message<String, Body<String, io::Error>>;
    type Response = Message<String, Body<String, io::Error>>;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let resp = Message::WithoutBody("Ok".to_string());

        match req {
            Message::WithoutBody(line) => {
                println!("{}", line);
                future::ok(resp).boxed()
            }
            Message::WithBody(_, body) => {
                let resp = body
                    .for_each(|line| {
                        println!(" + {}", line);
                        Ok(())
                    })
                    .map(move |_| resp);

                Box::new(resp) as Self::Future
            }
        }
    }
}
