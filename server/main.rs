#![feature(io_error_other)]

use std::ffi::CString;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::mem::size_of;
use std::net::{TcpListener, TcpStream};
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
#[repr(C, align(8))]
struct Header {
    magic: u32,
    kind: DataType,
}

const MAGIC: u32 = 0xdeadbeaf;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

const BUF_LEN: usize = 4096;

fn handle_connection(mut stream: TcpStream) -> Result<(), std::io::Error> {
    dbg!(&stream);
    // TODO: What is exactly read_timeout ?
    stream
        .set_read_timeout(Some(Duration::from_millis(200)))
        .unwrap();
    let mut buf_header = [0; size_of::<Header>()];
    stream.read_exact(&mut buf_header)?;
    let header: Header = unsafe { *(buf_header.as_ptr() as *const _) };
    dbg!(header);
    if header.magic != MAGIC {
        return Err(std::io::Error::other("Bad Magic Number"));
    }
    if header.kind != DataType::File {
        return Err(std::io::Error::other("Not a file header"));
    }
    let mut buf_file_header = [0; size_of::<FileHeader>()];
    stream.read_exact(&mut buf_file_header)?;
    let file_header: FileHeader = unsafe { *(buf_file_header.as_ptr() as *const _) };
    dbg!(file_header);
    let file_size = file_header.size as usize;
    let v = file_header.raw_name.to_vec();
    let zero_index = v.iter().position(|&b| b == 0).ok_or(std::io::Error::other(
        "File name must be terminated by zero",
    ))?;
    let file_name = CString::from_vec_with_nul(v[0..=zero_index].to_vec())
        .expect("Woot ?!?")
        .into_string()
        .map_err(|_| std::io::Error::other("Cannot convert to Rust String..."))?;

    dbg!(file_size);
    dbg!(&file_name);

    let mut file: File = File::create(file_name.as_str())?;
    let mut buf = [0; BUF_LEN];

    let mut readen_size = 0;
    while readen_size < file_size {
        let read_size: usize = if file_size - readen_size < BUF_LEN {
            file_size - readen_size
        } else {
            BUF_LEN
        };
        stream.read_exact(&mut buf[0..read_size])?;
        readen_size += read_size;
        file.write_all(&buf[0..read_size])?;
    }
    file.sync_all()?;
    match stream.read_exact(&mut buf[0..1]) {
        Ok(_) => Err(std::io::Error::other("EOF was not found !")),
        Err(_) => Ok(()),
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8021").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(e) = handle_connection(stream) {
                    eprintln!("{}", e);
                }
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}
