extern crate tokio_proto;
extern crate tokio_io;

use std::io;

use self::tokio_io::codec::Framed;
use self::tokio_io::{AsyncWrite, AsyncRead};
use self::tokio_proto::streaming::pipeline::ServerProto;

use line_codec::LineCodec;

struct LineProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> 
    for LineProto {

    type Request = String;
    type RequestBody = String;
    type Response = String;
    type ResponseBody = String;
    type Error = io::Error;

    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        let codec = LineCodec {
            decoding_head: true,
        };

        Ok(io.framed(codec))
    }
}

