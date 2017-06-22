use types_grpc::ABCIApplication;
use super::Service;
use std::net::{TcpListener, TcpStream};
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
/*
                    let mut buffer = [0; 8];
                    stream.read(&mut buffer).unwrap();
                    println!("slice: {:?}", &buffer);
                    let parsed = parse_from_bytes::<Request>(&buffer).unwrap();
                     */
                    let message = read_abci_message(stream);
                    let response = handle_abci_message(message, app);
                    write_abci_message(response);

                }
                Err(e) => {
                    println!("connection failed");
                }
            }
        });
    }

    Box::new(DummyApp{})
}

// reads the incoming stream and returns types::Request
// callers have to determine the concrete Request by matching against it and then respond appropriately
// and call the appropriate functions on the ABCIApplication
fn read_abci_message(stream: &mut TcpStream) -> types::Request {
    
}

// matches on the message and calls the appropriate functions on the app
// the function then returns appropriate response to write into the stream
fn handle_abci_message(message: , app: ) -> response {
    
}

// writes a protobuf message into the TcpStream
fn write_abci_message(stream: &mut TcpStream, response: ) 
