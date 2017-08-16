extern crate tokio_core;
extern crate futures;


use futures::{Future, BoxFuture, future, stream};
use futures::stream::Stream;
use tokio_core::reactor::Core;

#[allow(dead_code)]
fn ok_future() -> BoxFuture<u32, u32> {
    future::ok::<u32, u32>(3).boxed() 
}

#[allow(dead_code)]
fn enumerate_future() -> BoxFuture<(),()> {
    let my_stream = stream::iter::<_,u32,()>(
        (0 .. 20).map(|x| Ok(x))
    );

    my_stream
        .and_then(|x| {
            println!("{}", x*2);
            Ok(x)
        })
        .for_each(|x| {
            println!("Was in for_each {}", x);
            Ok(())
        })
        .boxed()
}

#[allow(dead_code)]
fn lift_result_future() -> BoxFuture<u32,()> {
    future::result(Ok(5)).boxed()
}


fn main() {
    let mut core = Core::new().unwrap();
    // let handle = core.handle();

    let my_future = enumerate_future();
    // let my_future = ok_future();
    // let my_future = lift_result_future();

    let val = core.run(my_future).unwrap();
    println!("{:?}", val);

}
