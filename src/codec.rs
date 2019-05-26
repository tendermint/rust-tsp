use std::error::Error;

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
    type Error = Box<Error>;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Request>, Box<Error>> {
        let length = buf.len();
        if length == 0 {
            return Ok(None);
        }
        let varint: (i64, usize) = i64::decode_var(&buf[..]);
        if varint.0 as usize + varint.1 > length {
            return Ok(None);
        }
        let request = protobuf::parse_from_bytes(&buf[varint.1..(varint.0 as usize + varint.1)])?;
        buf.split_to(varint.0 as usize + varint.1);
        Ok(Some(request))
    }
}

impl Encoder for ABCICodec {
    type Item = Response;
    type Error = Box<Error>;

    fn encode(&mut self, msg: Response, buf: &mut BytesMut) -> Result<(), Box<Error>> {
        let msg_len = msg.compute_size();
        let varint = i64::encode_var_vec(msg_len as i64);

        let remaining = buf.remaining_mut();
        let needed = msg_len as usize + varint.len();
        if remaining < needed {
            buf.reserve(needed);
        }

        buf.put(&varint);
        msg.write_to_writer(&mut buf.writer())?;
        trace!("Encode response! {:?}", &buf[..]);
        Ok(())
    }
}
