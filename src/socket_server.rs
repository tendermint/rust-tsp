use types_grpc::ABCIApplication;
use super::Service;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use protobuf::{Message, MessageStatic, parse_from_bytes};
use types::{Request, Response};
use types;
use byteorder::{ReadBytesExt, BigEndian};


struct DummyApp {}

impl Service for DummyApp {}

pub fn new_server<H: ABCIApplication + 'static + Sync + Send + 'static>(listen_addr: &str, app: H) -> Box<Service> {
    let server = TcpListener::bind(listen_addr).unwrap();

    for request in server.incoming() {
        thread::spawn(move || {
            println!("Received connection");
            match request {
                Ok(mut stream) => {
                    let request = read_abci_message(&mut stream).unwrap();
                    handle_abci_message(request);
                }
                Err(e) => {
                    println!("connection failed");
                }
            }
        });
    }

    Box::new(DummyApp{})
}

fn read_abci_message(stream: &mut TcpStream) -> Option<Request> {
    let varint_length = stream.read_u8().unwrap();
    if varint_length > 4 {
        return None;
    }

    let message_length: u64 = stream.read_uint::<BigEndian>(varint_length as usize).unwrap();

    let mut message_bytes: Vec<u8> = Vec::with_capacity(message_length as usize);

    stream.read_exact(&mut message_bytes);

    let message = parse_from_bytes::<Request>(&message_bytes);

    println!("{}", varint_length);
    println!("{}", message_length);
    println!("{:?}", &message);
    message.ok()
}


fn handle_abci_message(message: Request) {
    println!("handle_abci_message");
    if message.has_begin_block() {
        println!("begin_block");
    }

    if message.has_check_tx() {
        println!("check_tx");
    }

    if message.has_commit() {
        println!("commit");
    }

    if message.has_deliver_tx() {
        println!("deliver_tx");
    }

    if message.has_echo() {
        println!("echo");
    }

    if message.has_end_block() {
        println!("end_block");
    }

    if message.has_flush() {
        println!("flush");
    }

    if message.has_info() {
        println!("info");
    }

    if message.has_init_chain() {
        println!("init_chain");
    }

    if message.has_query() {
        println!("query");
    }

    if message.has_set_option() {
        println!("set_option");
    }

    println!("the fuck");
}

/*
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
*/
