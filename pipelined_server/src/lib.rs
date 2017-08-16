pub mod line_codec;
pub mod line_proto;
pub mod service;

extern crate tokio_io;
extern crate tokio_core;
extern crate tokio_service;
extern crate futures;


use std::io;

use tokio_core::reactor::Core;
use tokio_core::net::TcpListener;
use tokio_service::{Service, NewService};
use futures::{Future, Stream, Sink};
use tokio_io::AsyncRead;

use line_codec::LineCodec;

pub fn serve<S>(s: S, address: &str) -> io::Result<()>
    where S: NewService<Request = String,
                        Response = String,
                        Error = io::Error> + 'static
{
    let mut core = Core::new()?;
    let handle = core.handle();

    let address = address.parse().unwrap();
    let listener = TcpListener::bind(&address, &handle)?;

    let connections = listener.incoming();
    let server = connections.for_each(move |(socket, _peer_addr)| {
        let (writer, reader) = socket.framed(LineCodec).split();
        let service = s.new_service()?;

        let response = reader.and_then(move |req| service.call(req));
        let server = writer.send_all(response)
            .then(|_| Ok(()));
        handle.spawn(server);

        Ok(())
    });

    core.run(server)
}
