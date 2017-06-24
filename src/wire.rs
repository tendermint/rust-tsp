use std::net::TcpStream;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::Read;

// reads the encoded ABCI message from the stream
// returns a byteslice containing the encoded ABCI message
pub fn read_byte_slice(stream: &mut TcpStream) -> Vec<u8> {
    println!("read_byte_slice");
    // check for None and then just return None from this
    for byte in stream.bytes() {
        println!("{}", byte.unwrap());
    }
    /*
    let mut length: u64 = 0;
    {
        println!("inner");
        let mut stream_ref = stream.by_ref();
        length = read_varint(&mut stream_ref);
        println!("{}", length);
    }

    let mut buffer: Vec<u8> = Vec::new();
    stream.read_exact(&mut buffer);
    
    return buffer;
     */
    Vec::new()
}

// reads the first byte from the stream, which encodes the length of the BigEndian encoded length
// this deals with the prefix, it returns the length of the message
pub fn read_varint<S: Read>(stream: &mut S) -> u64 {
    let mut size = stream.read_u8().unwrap();
    println!("{}", &size);
    let mut i = stream.read_u64::<BigEndian>().unwrap();
    println!("{}", &i);
    return i;
}

#[test]
fn test_read_varint() {
    use std::io::Cursor;
    let mut buf = Cursor::new();
}

#[test]
fn _test_write_varint() {
    use std::io::Cursor;
    let mut buf = Cursor::new();
    
}

/*
// serializes a protobuf message into a byteslice and writes that slice to the stream
fn write_byte_slice(stream: &mut TcpStream) {
    write_varint();
    stream.write_all();
}

// writes the length byte of the ABCI message
fn write_varint() {
    
}
*/
