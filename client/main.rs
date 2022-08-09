use std::ffi::CString;
use std::io::prelude::*;
use std::mem::size_of;
use std::net::TcpStream;
use std::slice;

#[repr(C, align(8))]
struct Header {
    magic: u32,
    kind: DataType,
}

const MAGIC: u32 = 0xdeadbeaf;

#[repr(u32)]
enum DataType {
    File = 0,
}
#[derive(Debug, Copy, Clone)]
#[repr(C, align(8))]
struct FileHeader {
    size: u32,
    raw_name: [u8; 116],
}

fn main() -> std::io::Result<()> {
    assert_eq!(size_of::<Header>(), 8);
    assert_eq!(size_of::<Header>(), 8);
    let mut stream = TcpStream::connect("127.0.0.1:8021")?;

    stream.write(unsafe {
        slice::from_raw_parts(
            &Header {
                magic: MAGIC,
                kind: DataType::File,
            } as *const _ as *const u8,
            size_of::<Header>(),
        )
    })?;
    let name = CString::new("Carottes.jpg")
        .expect("CString::new failed")
        .into_bytes();
    let mut buf_name = [0; 116];
    name.iter().enumerate().for_each(|(i, b)| {
        buf_name[i] = *b;
    });
    stream.write(unsafe {
        slice::from_raw_parts(
            &FileHeader {
                size: 348912,
                raw_name: buf_name,
            } as *const _ as *const u8,
            size_of::<FileHeader>(),
        )
    })?;
    let v = vec![0xfa_u8; 348912];
    stream.write(&v)?;
    Ok(())
} // the stream is closed here
