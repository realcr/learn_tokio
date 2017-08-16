extern crate pipelined_server;
extern crate tokio_proto;

// use tokio_proto::TcpServer;

// use pipelined_server::line_proto::LineProto;
use pipelined_server::service::{/*Echo,*/ EchoRev};

use pipelined_server::serve;

fn main() {
    // let addr = "0.0.0.0:12345".parse().unwrap();
    // let server = TcpServer::new(LineProto, addr);

    println!("Server is listening on port 12345...");
    let _ = serve(|| Ok(EchoRev), "0.0.0.0:12345");
}
