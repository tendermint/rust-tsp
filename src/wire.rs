/*
Original go-wire implementation

func ReadByteSlice(r io.Reader, lmt int, n *int, err *error) []byte {
  length := ReadVarint(r, n, err)
  if *err != nil {
    return nil
	}
	if length < 0 {
		*err = ErrBinaryReadInvalidLength
		return nil
	}
	if lmt != 0 && lmt < cmn.MaxInt(length, *n+length) {
		*err = ErrBinaryReadOverflow
		return nil
	}

	buf := make([]byte, length)
	ReadFull(buf, r, n, err)
	return buf
}
 */

use std::net::TcpStream;
use byteorder::ReadBytesExt;

// reads the encoded ABCI message from the stream
// returns a byteslice containing the encoded ABCI message
fn read_byte_slice(stream: &mut TcpStream) -> Option<&[u8]> {
    // check for None and then just return None from this
    let length = read_varint(stream).unwrap();

    if length < 0 {
        None
    }

    let mut buffer = [0; length];

    stream.read_exact(buffer);

    return buffer;
}

// reads the first byte from the stream, which encodes the length of the BigEndian encoded length
// this deals with the prefix, it returns the length of the message
fn read_varint(stream: &mut TcpStream) -> Option<int> {
    let size = stream.read_u8().unwrap();
    let mut negate = false;

    if (size >> 4) == 0xF {
        negate = true;
        size = size & 0x0F;
    }

    if size > 8 {
        None
    }

    if size == 0 {
        if negate {
            None
        }
        Some(0)
    }

    let mut i = stream.read_u64::<BigEndian>().unwrap();
    if negate {
        return Some(-i);
    } else {
        return Some(i);
    }

}

// serializes a protobuf message into a byteslice and writes that slice to the stream
fn write_byte_slice(stream: &mut TcpStream) {
    write_varint();
    stream.write_all();
}

// writes the length byte of the ABCI message
fn write_varint() {
    
}

