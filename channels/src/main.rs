extern crate tokio_core;
extern crate futures;

use std::io::{self, BufRead};
use std::thread;

use tokio_core::reactor::Core;
use futures::{Stream, Sink, Future};
use futures::stream::BoxStream;
use futures::sync::mpsc;

fn stdin() -> BoxStream<String, io::Error> {
    let (mut tx, rx) = mpsc::channel(1);

    thread::spawn(|| {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match tx.send(line).wait() {
                Ok(t) => tx = t,
                Err(_) => break,
            }
        }
    });

    rx.then(|r| r.unwrap()).boxed()
}


fn main() {

    let mut core = Core::new().unwrap();
    let stdin_stream = stdin();

    let server = stdin_stream.for_each(move |line| {
        println!("line = {}",line);
        Ok(())
    });

    // let val = server.wait();

    let val = core.run(server);
    println!("val = {:?}", val);
}

