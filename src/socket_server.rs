use types_grpc::ABCIApplication;
use super::Service;
use std::net::TcpListener;
use std::thread;
use std::io::Read;
use websocket::sync::Server;
use websocket::server::NoTlsAcceptor;
use protobuf::Message;
use protobuf::MessageStatic;
use protobuf::parse_from_bytes;
use types::Request;


pub struct SocketServer<H> {
    server: Server<NoTlsAcceptor>,
    app: H,
}

impl<H: ABCIApplication + 'static + Sync + Send + 'static> Service for SocketServer<H> {}

struct DummyApp {}

impl Service for DummyApp {}

pub fn new_server<H: ABCIApplication + 'static + Sync + Send + 'static>(listen_addr: &str, app: H) -> Box<Service> {
    let server = TcpListener::bind(listen_addr).unwrap();

    for request in server.incoming() {
        thread::spawn(move || {
            println!("Received connection");
            match request {
                Ok(mut stream) => {
                    // TODO: determine how long the message is based on the first byte in the buffer
                    let mut buffer = [0; 8];
                    stream.read(&mut buffer).unwrap();
                    println!("slice: {:?}", &buffer);
                    let parsed = parse_from_bytes::<Request>(&buffer).unwrap();
                }
                Err(e) => {
                    println!("connection failed");
                }
            }
        });
    }

    Box::new(DummyApp{})
}
