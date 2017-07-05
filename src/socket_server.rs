use std::io;
use std::str;
use bytes;
use bytes::{BytesMut, ByteOrder, BigEndian};
use tokio_io::codec::{Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use types;
use byteorder::{ReadBytesExt};
use protobuf;


pub struct ABCICodec;

impl Decoder for ABCICodec {
    type Item = types::Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<types::Request>> {
        println!("{:?}", buf.as_ref());
        let varint_length_bytes = buf.split_to(1);
        println!("{:?}", varint_length_bytes);

        let varint_length = varint_length_bytes[0] as usize;
        println!("{:?}", varint_length);

        let message_length_bytes = buf.split_to(varint_length);
        println!("{:?}", message_length_bytes);

        let message_length: u64 = BigEndian::read_uint(&message_length_bytes, varint_length);
        println!("{:?}", message_length);

        let message_bytes = buf.split_to(message_length as usize);
        println!("{:?}", message_bytes);

        let message = protobuf::core::parse_from_bytes::<types::Request>(&message_bytes);
        println!("{:?}", message);

        Ok(message.ok())
    }
}

impl Encoder for ABCICodec {
    type Item = types::Response;
    type Error = io::Error;

    fn encode(&mut self, msg: types::Response, buf: &mut BytesMut) -> io::Result<()> {
        println!("{:?}", msg);
        Ok(())
    }
}


pub struct ABCIProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for ABCIProto {
    type Request = types::Request;

    type Response = types::Response;

    type Transport = Framed<T, ABCICodec>;

    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(ABCICodec))
    }
}


// Needs a field to hold the ABCI app that runs through this service
pub struct ABCIService {
    
}

impl Service for ABCIService {
    type Request = types::Request;
    type Response = types::Response;

    type Error = io::Error;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::err(io::Error::new(io::ErrorKind::Other, "nothing happened yet")).boxed()
    }
}


// The app should definitely not be Send + Sync
// it should be protected by a mutex to only allow sequential and ordered read
// and write access
pub fn new_server<H>(listen_addr: &str, app: H) {
}




/*
fn write_abci_message(stream: &mut TcpStream, response: &mut Response) {
    let message = response.write_to_bytes().unwrap();
    let message_length = message.len();

    let varint_length = (7+64-message_length.leading_zeros())/8;

    stream.write_u8(varint_length as u8).unwrap();

    stream.write_u64::<BigEndian>(message_length as u64).unwrap();

    stream.write_all(&message).unwrap();

    stream.flush().unwrap();

    println!("{:?}", &varint_length);
    println!("{:?}", &message_length);
    println!("{:?}", &message);
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

    stream.read_exact(&mut message_bytes).unwrap();

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
*/
