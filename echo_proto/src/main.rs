extern crate echo_proto;
extern crate tokio_proto;

use tokio_proto::TcpServer;

use echo_proto::line_proto::LineProto;
use echo_proto::service::Echo;

fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(LineProto, addr);

    println!("Server is listening on port 12345...");
    server.serve(|| Ok(Echo));
}
