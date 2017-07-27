use std::io;
use std::io::Write;
use std::str;
use bytes;
use bytes::{BytesMut, ByteOrder, BigEndian, BufMut};
use tokio_io::codec::{Encoder, Decoder};
use tokio_proto::pipeline::ServerProto;
use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::Framed;
use tokio_service::Service;
use futures::{future, Future, BoxFuture};
use types;
use byteorder::{ReadBytesExt, WriteBytesExt};
use protobuf;
use protobuf::Message;
use std::cmp;
use std::sync::Arc;


pub struct ABCICodec;

impl Decoder for ABCICodec {
    type Item = types::Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<types::Request>> {
        let avail = buf.len();
        if avail == 0 {
            return Ok(None);
        }

        let varint_len = buf[0] as usize;
        if varint_len == 0 || varint_len > 8 {
            return Err(io::Error::new(io::ErrorKind::InvalidData,
            "bogus packet length"));
        }

        let msg_nbytes = BigEndian::read_uint(&buf[1 .. (varint_len + 1)], varint_len) as usize;
        let header_len = 1 + varint_len;

        if (avail - header_len) < msg_nbytes {
            return Ok(None);
        }

        print!(" >> ");
        for b in &buf[.. avail] {
            print!("{:02x} ", b);
        }
        println!();

        let message = protobuf::core::parse_from_bytes(
            &buf[header_len .. (header_len + msg_nbytes)]);
        let _ = buf.split_to(header_len + msg_nbytes);

        println!("{:?}", varint_len);
        println!("{:?}", msg_nbytes);
        println!("{:?}", &message);

        return Ok(message.ok());
    }
}

impl Encoder for ABCICodec {
    type Item = types::Response;
    type Error = io::Error;

    fn encode(&mut self, msg: types::Response, buf: &mut BytesMut) -> io::Result<()> {
        let msg_len = msg.compute_size();
        let varint_len = cmp::max(8 - ((msg_len as u64).leading_zeros() >> 3), 1);
        let total_msg_len = (1 + varint_len + msg_len) as usize;

        buf.reserve(total_msg_len);

        let mut writer = buf.writer();

        let msg_len_bytes = {
            let mut buf = [0u8; 8];
            BigEndian::write_u64(&mut buf, msg_len as u64);
            buf
        };

        writer.write_u8(varint_len as u8)?;
        writer.write(&msg_len_bytes[(8 - varint_len as usize) ..])?;
        msg.write_to_writer(&mut writer).unwrap();

        print!(" << ");
        for b in &writer.into_inner()[.. total_msg_len] {
            print!("{:02x} ", b);
        }
        println!();

        println!("{:?}", varint_len);
        println!("{:?}", msg_len as u64);
        println!("{:?}", &msg);

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
    pub app: Box<Application>
}

impl Service for ABCIService {
    type Request = types::Request;
    type Response = types::Response;

    type Error = io::Error;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let res = handle_abci_message(&req, &self.app);
        return future::ok(res).boxed();
    }
}

pub trait Application {
    fn begin_block(&self, p: &types::RequestBeginBlock) -> types::ResponseBeginBlock;

    fn check_tx(&self, p: &types::RequestCheckTx) -> types::ResponseCheckTx;

    fn commit(&self, p: &types::RequestCommit) -> types::ResponseCommit;

    fn deliver_tx(&self, p: &types::RequestDeliverTx) -> types::ResponseDeliverTx;

    fn echo(&self, p: &types::RequestEcho) -> types::ResponseEcho;

    fn end_block(&self, p: &types::RequestEndBlock) -> types::ResponseEndBlock;

    fn flush(&self, p: &types::RequestFlush) -> types::ResponseFlush;

    fn info(&self, p: &types::RequestInfo) -> types::ResponseInfo;

    fn init_chain(&self, p: &types::RequestInitChain) -> types::ResponseInitChain;

    fn query(&self, p: &types::RequestQuery) -> types::ResponseQuery;

    fn set_option(&self, p: &types::RequestSetOption) -> types::ResponseSetOption;
}

fn handle_abci_message(message: &types::Request, app: &Box<Application>) -> types::Response {
    let mut result = types::Response::new();
    if message.has_begin_block() {
        let response = app.begin_block(message.get_begin_block());
        result.set_begin_block(response);
        return result;
    }

    if message.has_check_tx() {
        let response = app.check_tx(message.get_check_tx());
        result.set_check_tx(response);
        return result;
    }

    if message.has_commit() {
        let response = app.commit(message.get_commit());
        result.set_commit(response);
        return result;
    }

    if message.has_deliver_tx() {
        let response = app.deliver_tx(message.get_deliver_tx());
        result.set_deliver_tx(response);
        return result;
    }

    if message.has_echo() {
        let response = app.echo(message.get_echo());
        result.set_echo(response);
        return result;
    }

    if message.has_end_block() {
        let response = app.end_block(message.get_end_block());
        result.set_end_block(response);
        return result;
    }

    if message.has_flush() {
        let response = app.flush(message.get_flush());
        result.set_flush(response);
        return result;
    }

    if message.has_info() {
        let response = app.info(message.get_info());
        result.set_info(response);
        return result;
    }

    if message.has_init_chain() {
        let response = app.init_chain(message.get_init_chain());
        result.set_init_chain(response);
        return result;
    }

    if message.has_query() {
        let response = app.query(message.get_query());
        result.set_query(response);
        return result;
    }

    if message.has_set_option() {
        let response = app.set_option(message.get_set_option());
        result.set_set_option(response);
        return result;
    }
    return result;
}
