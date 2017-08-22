extern crate bytes;
extern crate futures;
extern crate tokio_io;
extern crate tokio_proto;

use std::io;
use self::tokio_proto::pipeline::ServerProto;
use self::tokio_io::{AsyncRead, AsyncWrite};
use self::tokio_io::codec::Framed;
use self::futures::{Future, future, Stream, Sink};
use self::futures::stream::SplitSink;


use line_codec::LineCodec;

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, LineCodec>;
    // type BindTransport = BoxFuture<Self::Transport, io::Error>;
    type BindTransport = Box<Future<Item = Self::Transport, Error = io::Error>>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        // let transport = io.framed(LineCodec);
        let transport = io.framed(LineCodec);

        // .send("You ready?".into())
        //
        /*
         *fn reunite(self, other: SplitSink<S>) -> Result<S, ReuniteError<S>>
         */


        Box::new(transport.send("You ready?".into())
            .and_then(|transport| {
                transport.into_future()
            })
            .map_err(|(e, _)| e)
            .and_then(|(line, transport)| {
                match line {
                    Some(ref msg) if msg == "Bring it!" => {
                        println!("SERVER: received client handshake");
                        // let ret = transport.send("Bring it!".into());
                        Box::new(future::ok(transport)) as Self::BindTransport
                    }
                    _ => {
                        println!("SERVER: client handshare INVALID");
                        let err = io::Error::new(io::ErrorKind::Other,
                                                 "invalid handshake");
                        let ret = future::err(err);
                        Box::new(ret) as Self::BindTransport
                    }
                }
            })

        )
    }
}


