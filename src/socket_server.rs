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


pub trait Application {
    fn begin_block(&self, p: types::RequestBeginBlock) -> types::ResponseBeginBlock;

    fn check_tx(&self, p: types::RequestCheckTx) -> types::ResponseCheckTx;

    fn commit(&self, p: types::RequestCommit) -> types::ResponseCommit;

    fn deliver_tx(&self, p: types::RequestDeliverTx) -> types::ResponseDeliverTx;

    fn echo(&self, p: types::RequestEcho) -> types::ResponseEcho;

    fn end_block(&self, p: types::RequestEndBlock) -> types::ResponseEndBlock;

    fn flush(&self, p: types::RequestFlush) -> types::ResponseFlush;

    fn info(&self, p: types::RequestInfo) -> types::ResponseInfo;

    fn init_chain(&self, p: types::RequestInitChain) -> types::ResponseInitChain;

    fn query(&self, p: types::RequestQuery) -> types::ResponseQuery;

    fn set_option(&self, p: types::RequestSetOption) -> types::ResponseSetOption;
}

pub fn new_server<H: Application + 'static + Send>(listen_addr: &str, app: Box<H>) -> Box<Service> {
    let server = TcpListener::bind(listen_addr).unwrap();

    for request in server.incoming() {
        thread::spawn(move || {
            match request {
                Ok(mut stream) => {
                    let request = read_abci_message(&mut stream).unwrap();
                    handle_abci_message(&mut stream, request, app.clone());
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

    let mut message_bytes: Vec<u8> = vec![0; message_length as usize];

    stream.read_exact(&mut message_bytes);

    let message = parse_from_bytes::<Request>(&message_bytes);

    message.ok()
}

fn write_abci_message(stream: &mut TcpStream) {
    
}


fn handle_abci_message<H: Application + 'static + Send>(stream: &mut TcpStream, message: Request, app: Box<H>) {
    if message.has_begin_block() {
        let response = app.begin_block(*message.get_begin_block());
    }

    if message.has_check_tx() {
        let response = app.check_tx(*message.get_check_tx());
    }

    if message.has_commit() {
        let response = app.commit(*message.get_commit());
    }

    if message.has_deliver_tx() {
        let response = app.deliver_tx(*message.get_deliver_tx());
    }

    if message.has_echo() {
        let response = app.echo(*message.get_echo());
    }

    if message.has_end_block() {
        let response = app.end_block(*message.get_end_block());
    }

    if message.has_flush() {
        let response = app.flush(*message.get_flush());
    }

    if message.has_info() {
        let response = app.info(*message.get_info());
    }

    if message.has_init_chain() {
        let response = app.init_chain(*message.get_init_chain());
    }

    if message.has_query() {
        let response = app.query(*message.get_query());
    }

    if message.has_set_option() {
        let response = app.set_option(*message.get_set_option());
    }
}
