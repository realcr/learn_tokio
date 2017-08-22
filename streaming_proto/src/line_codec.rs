extern crate bytes;
extern crate tokio_proto;
extern crate tokio_io;

use std::io;
use std::str;

use self::tokio_proto::streaming::pipeline::Frame;
use self::bytes::{BytesMut};
use self::tokio_io::codec::{Encoder, Decoder};


pub struct LineCodec {
    pub decoding_head: bool,
}

impl Decoder for LineCodec {
    type Item = Frame<String, String, io::Error>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut)
        -> Result<Option<Self::Item>, io::Error>
    {
        // Find the next newline character:
        let line = match buf.iter()
            .position(|b| *b == b'\n') {

            Some(n) => buf.split_to(n),
            None => return Ok(None),
        };
        // Remove the newline:
        buf.split_to(1);

        let s = try!(str::from_utf8(&line).map_err(|e| {
            io::Error::new(io::ErrorKind::Other, e)
        }));

        if s == "" {
            let decoding_head = self.decoding_head;
            // Toggle decoding_head state:
            self.decoding_head = !decoding_head;

            if decoding_head {
                Ok(Some(Frame::Message {
                    message: s.to_string(),
                    body: true,
                }))
            } else {
                Ok(Some(Frame::Body {
                    chunk: None
                }))
            }
        } else {
            if self.decoding_head {
                Ok(Some(Frame::Message {
                    message: s.to_string(),
                    body: false,
                }))
            } else {
                Ok(Some(Frame::Body {
                    chunk: Some(s.to_string()),
                }))
            }
        }
    }
}

impl Encoder for LineCodec {
    type Item = Frame<String, String, io::Error>;
    type Error = io::Error;

    fn encode(&mut self, msg: Self::Item, buf: &mut BytesMut) 
        -> io::Result<()> {

        match msg {
            Frame::Message { message, body } => {
                assert!(message.is_empty() == body);
                buf.extend(message.as_bytes());
            }
            Frame::Body { chunk } => {
                if let Some(chunk) = chunk {
                    buf.extend(chunk.as_bytes())
                }
            }
            Frame::Error { error } => {
                return Err(error);
            }
        }

        // A final newline character:
        buf.extend(b"\n");

        Ok(())
    }
}
