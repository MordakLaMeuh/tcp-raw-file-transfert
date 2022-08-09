### Raw TCP file server

## metadata

# Header: 8 bytes

- 4 bytes for MAGIC: 0xdeadbeaf
- 4 bytes for TYPE: 0x00000000

# File Header: 120 bytes

- 4 bytes for file size
- 116 bytes for file name, must be terminated by \0 like C Style

Then all the file content

# Commands

`rustup update`

`rustup default nightly`

`cargo build`

`cargo run --bin server`

# x86 and IA32 use Little Endian
# listening on TCP port 8021
