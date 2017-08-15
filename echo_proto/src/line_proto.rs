extern crate bytes;
extern crate tokio_io;
extern crate tokio_proto;

use std::io;
use self::tokio_proto::pipeline::ServerProto;
use self::tokio_io::{AsyncRead, AsyncWrite};
use self::tokio_io::codec::Framed;

use line_codec::LineCodec;

pub struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for LineProto {
    type Request = String;
    type Response = String;
    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(LineCodec))
    }
}


