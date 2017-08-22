extern crate streaming_proto;
extern crate tokio_proto;

use self::tokio_proto::TcpServer;
use streaming_proto::line_proto::LineProto;
use streaming_proto::service::PrintStdout;


fn main() {
    let addr = "0.0.0.0:12345".parse().unwrap();
    let server = TcpServer::new(LineProto, addr);
    println!("Listening on port 12345 ...");
    server.serve(|| Ok(PrintStdout));
}
