extern crate bytes;
extern crate tokio_proto;
extern crate tokio_io;

use std::io;
use std::str;

use self::tokio_proto::multiplex::RequestId;
use self::bytes::{BytesMut, BigEndian, IntoBuf, Buf, BufMut};
use self::tokio_io::codec::{Encoder, Decoder};

pub struct LineCodec;

impl Decoder for LineCodec {
    type Item = (RequestId, String);
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut)
        -> io::Result<Option<(RequestId, String)>>
    {
        // We need at least 5 bytes for a full message:
        if buf.len() < 5 {
            // We don't have a full message yet.
            return Ok(None);
        }

        // Find the newline chatacter:
        let newline = buf[4..].iter()
            .position(|b| *b == b'\n');

        if let Some(n) = newline {
            let mut line = buf.split_to(n + 4);

            // Remove the newline character:
            buf.split_to(1);

            let id = line.split_to(4).into_buf()
                .get_u32::<BigEndian>();

            return match str::from_utf8(&line[..]) {
                Ok(s) => Ok(Some((id as RequestId, s.to_string()))),
                Err(_) => Err(io::Error::new(io::ErrorKind::Other,
                                             "invalid string")),
            }

        }

        // We couldn't find a newline character:
        Ok(None)
    }
}

impl Encoder for LineCodec {
    type Item = (RequestId, String);
    type Error = io::Error;

    fn encode(&mut self,
              msg: (RequestId, String),
              buf: &mut BytesMut) -> io::Result<()>
    {
        let (id, msg) = msg;

        buf.put_u32::<BigEndian>(id as u32);
        buf.put(msg.as_bytes());
        buf.put("\n");

        Ok(())
    }
}

