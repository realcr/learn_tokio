extern crate futures;
extern crate tokio_core;
extern crate tokio_io;

use futures::stream::Stream;
use futures::Future;
use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;

#[allow(dead_code)]
fn sequential_server()  {
    let mut core = Core::new().unwrap();
    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &core.handle())
        .unwrap();

    let connections = listener.incoming();
    let welcomes = connections.and_then(|(socket, _peer_addr)| {
        tokio_io::io::write_all(socket, b"Hello, world!\n")
    });

    let server = welcomes.for_each(|(_socket, _welcome)| {
        Ok(())
    });

    core.run(server).unwrap()
}

#[allow(dead_code)]
fn concurrent_server() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();


    let address = "0.0.0.0:12345".parse().unwrap();
    let listener = TcpListener::bind(&address, &core.handle())
        .unwrap();

    let connections = listener.incoming();

    let server = connections.for_each(|(socket, _peer_addr)| {
        let serve_one = tokio_io::io::write_all(socket, b"Hello, world!\n")
            .then(|_| Ok(()));
        handle.spawn(serve_one);
        Ok(())
    });

    core.run(server).unwrap()

}

fn main() {

    println!("Listening on port 12345...");
    // sequential_server();
    concurrent_server();
}
