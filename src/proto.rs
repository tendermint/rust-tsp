pub mod abci;
pub mod merkle;
pub mod types;

use std::io::{Error, ErrorKind, Result};

#[cfg(feature = "async-std")]
use async_std::{
    io::{Read, Write},
    prelude::*,
};
#[cfg(feature = "tokio")]
use tokio::io::{AsyncRead as Read, AsyncReadExt, AsyncWrite as Write, AsyncWriteExt};

use integer_encoding::VarInt;
use protobuf::{parse_from_bytes, Message};

use self::abci::{Request, Response};

const BUFLEN: usize = 10;
const MSB: u8 = 0b1000_0000;

/// Decodes a `Request` from stream
pub async fn decode<R: Read + Unpin>(mut reader: R) -> Result<Option<Request>> {
    let length: i64 = read_varint(&mut reader).await?;

    if length == 0 {
        return Ok(None);
    }

    let mut bytes = vec![0; length as usize];
    reader.take(length as u64).read(&mut bytes).await?;

    parse_from_bytes(&bytes)
        .map(Some)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}

/// Encodes a `Response` to stream
pub async fn encode<W: Write + Unpin>(message: Response, mut writer: W) -> Result<()> {
    write_varint(&mut writer, i64::from(message.compute_size())).await?;

    let bytes = message
        .write_to_bytes()
        .map_err(|e| Error::new(ErrorKind::Other, e))?;

    writer.write_all(&bytes).await
}

/// Reads a varint from `AsyncRead`. Implementation is based on original synchronous version of
/// [`read_varint`](https://github.com/dermesser/integer-encoding-rs/blob/v1.0.7/src/reader.rs#L21)
///
/// There won't be any need for this once [this](https://github.com/dermesser/integer-encoding-rs/issues/4) is fixed
async fn read_varint<VI: VarInt, R: Read + Unpin>(mut reader: R) -> Result<VI> {
    let mut buf = [0 as u8; BUFLEN];
    let mut i = 0;

    loop {
        if i >= BUFLEN {
            return Err(Error::new(ErrorKind::InvalidData, "Unterminated varint"));
        }

        let read = reader.read(&mut buf[i..=i]).await?;

        // EOF
        if read == 0 && i == 0 {
            return Err(Error::new(ErrorKind::UnexpectedEof, "Reached EOF"));
        }

        if buf[i] & MSB == 0 {
            break;
        }

        i += 1;
    }

    let (result, _) = VI::decode_var(&buf[0..=i]);

    Ok(result)
}

/// Writes a varint to `AsyncWrite`. Implementation is based on original synchronous version of
/// [`write_varint`](https://github.com/dermesser/integer-encoding-rs/blob/v1.0.7/src/writer.rs#L12)
///
/// There won't be any need for this once [this](https://github.com/dermesser/integer-encoding-rs/issues/4) is fixed
async fn write_varint<VI: VarInt, W: Write + Unpin>(mut writer: W, n: VI) -> Result<usize> {
    let mut buf = [0 as u8; BUFLEN];
    let used = n.encode_var(&mut buf[..]);

    writer.write(&buf[0..used]).await
}
