use std::io;

use bytes::{BufMut, BytesMut};
use integer_encoding::VarInt;
use protobuf::Message;
use tokio::codec::{Decoder, Encoder};

use messages::abci::*;

#[derive(Debug)]
pub struct ABCICodec;

impl ABCICodec {
    pub fn new() -> ABCICodec {
        ABCICodec
    }
}

impl Decoder for ABCICodec {
    type Item = Request;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Request>> {
        let length = buf.len();
        if length == 0 {
            return Ok(None);
        }
        let varint: (i64, usize) = i64::decode_var(&buf[..]);
        if varint.0 as usize + varint.1 > length {
            return Ok(None);
        }
        let message = protobuf::parse_from_bytes(&buf[varint.1..(varint.0 as usize + varint.1)]);
        buf.split_to(varint.0 as usize + varint.1);
        Ok(message.ok())
    }
}

impl Encoder for ABCICodec {
    type Item = Response;
    type Error = io::Error;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> io::Result<()> {
        let mut msg_to_vec = Vec::new();
        msg.write_to_vec(&mut msg_to_vec).unwrap();
        let msg_len: i64 = msg_to_vec.len() as i64;
        let varint = i64::encode_var_vec(msg_len);
        buf.put(&varint);
        buf.reserve(1 + msg_len as usize);
        msg.write_to_writer(&mut buf.writer()).unwrap();
        info!("Encode response! {:?}", &buf[..]);
        Ok(())
    }
}
