#![feature(conservative_impl_trait)]

extern crate tokio_core;
extern crate futures;

use std::fmt;

use tokio_core::reactor::Core;
use futures::{Stream, Future};
use futures::{stream};

/// Generate a stream of numbers from 0 .. x
fn gen_stream(x: usize) -> impl Stream<Item=usize, Error=()> {
    let my_stream = stream::iter::<_,usize,()>(
        (0 .. x).map(|x| Ok(x))
    );
    my_stream
}

/// A future for printing a whole stream.
fn print_stream<S,T,E>(stream: S) -> impl Future<Item=(), Error=E>
    where S: Stream<Item=T, Error=E>,
          T: fmt::Display {

    stream.for_each(|x| {
        // Print each element:
        print!("{} ", x);
        Ok(())
    }).and_then(|_| {
        // After all elements were printed, we print a newline:
        println!();
        Ok(())
    })
}

fn main() {

    let mut core = Core::new().unwrap();
    let my_stream = gen_stream(100);

    let my_future = my_stream
        .into_future()
        .and_then(|(opt_num, my_stream)| {
            match opt_num {
                Some(num) => println!("num = {}", num),
                None => println!("No elements to read!"),
            };
            Ok(my_stream)
        }).map_err(|(e, _)| e) 
        .and_then(|my_stream| {
            print_stream(my_stream)
        });

    core.run(my_future).unwrap()
}


