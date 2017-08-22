extern crate conn_handshake;
extern crate tokio_proto;

use tokio_proto::TcpServer;

use conn_handshake::line_proto::LineProto;
use conn_handshake::service::{Echo};

fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(LineProto, addr);

    println!("Server is listening on port 12345...");
    server.serve(|| Ok(Echo));
}
