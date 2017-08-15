use std::cmp;
use std::io;
use std::io::Write;
use std::net::SocketAddr;

use types::*;
use super::Application;


use bytes::{BigEndian, BytesMut, ByteOrder, BufMut};

use byteorder::WriteBytesExt;

use futures::future;
use futures::{BoxFuture, Future};

use protobuf;
use protobuf::Message;

use tokio_io::{AsyncRead, AsyncWrite};
use tokio_io::codec::{Decoder, Encoder, Framed};

use tokio_proto::TcpServer;
use tokio_proto::pipeline::ServerProto;

use tokio_service::Service;


pub fn new<A: Application + Send + Sync + 'static>(listen_addr: SocketAddr, app: &'static A) {
    let server = TcpServer::new(TSPProto, listen_addr);
    server.serve(move|| Ok(TSPService{app: app}));
}

// A codec describes how to go from a bunch of bytes from the wire into a
// deserialised request. The codec handles the deserialisation from buffer
// to request as defined in types.proto
struct TSPCodec;

impl Decoder for TSPCodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        let avail = buf.len();
        if avail == 0 {
            return Ok(None);
        }

        let varint_len = buf[0] as usize;
        if varint_len == 0 || varint_len > 8 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "bogus packet length"));
        }

        if avail < varint_len+1 {
            return Ok(None);
        }

        let msg_nbytes = BigEndian::read_uint(&buf[1 .. (varint_len + 1)], varint_len) as usize;
        let header_len = 1 + varint_len;

        if (avail - header_len) < msg_nbytes {
            return Ok(None);
        }

        let message = protobuf::core::parse_from_bytes(
            &buf[header_len .. (header_len + msg_nbytes)]);
        let _ = buf.split_to(header_len + msg_nbytes);

        return Ok(message.ok());
    }
}

impl Encoder for TSPCodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
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

        Ok(())
    }
}


struct TSPProto;

impl<T: AsyncRead + AsyncWrite + 'static> ServerProto<T> for TSPProto {
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, TSPCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(TSPCodec))
    }
}


struct TSPService {
    app: &'static Application
}

impl Service for TSPService {
    type Request = Request;
    type Response = Response;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let response = self.handle(&req);
        future::ok(response).boxed()
    }
}

impl TSPService {
    fn handle(&self, request: &Request) -> Response {
        let mut response = Response::new();

        // Info/Query connection
        if request.has_info() {
            response.set_info(self.app.info(request.get_info()));
            return response;
        }

        if request.has_set_option() {
            response.set_set_option(self.app.set_option(request.get_set_option()));
            return response;
        }

        if request.has_query() {
            response.set_query(self.app.query(request.get_query()));
            return response;
        }

        // Mempool connection
        if request.has_check_tx() {
            response.set_check_tx(self.app.check_tx(request.get_check_tx()));
            return response;
        }

        // Consensus connection
        if request.has_init_chain() {
            response.set_init_chain(self.app.init_chain(request.get_init_chain()));
            return response;
        }

        if request.has_begin_block() {
            response.set_begin_block(self.app.begin_block(request.get_begin_block()));
            return response;
        }

        if request.has_deliver_tx() {
            response.set_deliver_tx(self.app.deliver_tx(request.get_deliver_tx()));
            return response;
        }

        if request.has_end_block() {
            response.set_end_block(self.app.end_block(request.get_end_block()));
            return response;
        }

        if request.has_commit() {
            response.set_commit(self.app.commit(request.get_commit()));
            return response;
        }

        // Miscellaneous connection
        if request.has_echo() {
            response.set_echo(self.app.echo(request.get_echo()));
            return response;
        }

        if request.has_flush() {
            response.set_flush(self.app.flush(request.get_flush()));
            return response;
        }

        unreachable!();
    }
}
