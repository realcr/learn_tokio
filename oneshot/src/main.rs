extern crate futures;

use std::thread;
use std::time::Duration;

use futures::{Future, future};
use futures::sync::oneshot;

#[allow(dead_code)]
fn expensive_computation() -> u32 {
    200
}

#[allow(dead_code)]
fn basic_oneshot() {
    let (tx, rx) = oneshot::channel();

    thread::spawn(move || {
        tx.send(expensive_computation());
    });

    let rx = rx.map(|x| x + 3);
    let result = rx.wait().unwrap();
    assert_eq!(result, 203);
}

#[allow(dead_code)]
fn cancel_oneshot() {
    let (mut tx, rx) = oneshot::channel::<()>();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(20));
        drop(rx);
    });
    let future = future::poll_fn(|| tx.poll_cancel());
    future.wait().unwrap();


}

fn main() {
    // basic_oneshot();
    cancel_oneshot();
}
