use types_grpc::ABCIApplication;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Read, Write};
use protobuf::{Message, MessageStatic, parse_from_bytes};
use types::{Request, Response};
use types;
use byteorder::{ReadBytesExt, BigEndian, WriteBytesExt};
use std::sync::Arc;

pub fn new_server<H: Application + 'static + Send + Sync + 'static>(listen_addr: &str, app: H) {
    let server = TcpListener::bind(listen_addr).unwrap();
    let app = Arc::new(app);

    for request in server.incoming() {
        let clone = app.clone();
        thread::spawn(move || {
            match request {
                Ok(mut stream) => {
                    let mut request = read_abci_message(&mut stream).unwrap();
                    let response = handle_abci_message(&mut request, clone);
                    write_abci_message(&mut stream, response);
                }
                Err(e) => {
                    println!("connection failed");
                }
            }
        });
    }
}

fn write_abci_message(stream: &mut TcpStream, response: Response) {
    let bytes = response.write_to_bytes().unwrap();

    let i = bytes.len();

    let size = (7+64-i.leading_zeros())/8;

    stream.write_u8(size as u8);

    stream.write_u64::<BigEndian>(i as u64);

    stream.write_all(&bytes);

    stream.flush();

    println!("{:?}", &response);
    println!("{:?}", &bytes);
    println!("{:?}", i);
    println!("{:?}", size);
}

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

fn read_abci_message(stream: &mut TcpStream) -> Option<Request> {
    let varint_length = stream.read_u8().unwrap();
    if varint_length > 4 {
        return None;
    }

    let message_length: u64 = stream.read_uint::<BigEndian>(varint_length as usize).unwrap();

    let mut message_bytes: Vec<u8> = vec![0; message_length as usize];

    stream.read_exact(&mut message_bytes);

    let message = parse_from_bytes::<Request>(&message_bytes);

    println!("{:?}", varint_length);
    println!("{:?}", message_length);
    println!("{:?}", message_bytes);
    println!("{:?}", &message);

    message.ok()
}

fn handle_abci_message<H: Application + 'static + Send + Sync + 'static>(message: &mut Request, app: Arc<H>) -> Response {
    let mut result = Response::new();
    if message.has_begin_block() {
        let response = app.begin_block(message.take_begin_block());
        result.set_begin_block(response);
        return result;
    }

    if message.has_check_tx() {
        let response = app.check_tx(message.take_check_tx());
        result.set_check_tx(response);
        return result;
    }

    if message.has_commit() {
        let response = app.commit(message.take_commit());
        result.set_commit(response);
        return result;
    }

    if message.has_deliver_tx() {
        let response = app.deliver_tx(message.take_deliver_tx());
        result.set_deliver_tx(response);
        return result;
    }

    if message.has_echo() {
        let response = app.echo(message.take_echo());
        result.set_echo(response);
        return result;
    }

    if message.has_end_block() {
        let response = app.end_block(message.take_end_block());
        result.set_end_block(response);
        return result;
    }

    if message.has_flush() {
        let response = app.flush(message.take_flush());
        result.set_flush(response);
        return result;
    }

    if message.has_info() {
        let response = app.info(message.take_info());
        result.set_info(response);
        return result;
    }

    if message.has_init_chain() {
        let response = app.init_chain(message.take_init_chain());
        result.set_init_chain(response);
        return result;
    }

    if message.has_query() {
        let response = app.query(message.take_query());
        result.set_query(response);
        return result;
    }

    if message.has_set_option() {
        let response = app.set_option(message.take_set_option());
        result.set_set_option(response);
        return result;
    }
    return result;
}
