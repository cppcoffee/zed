
use std::os::fd::{AsRawFd, FromRawFd};
use std::fs::File;
use std::io::Read;

fn main() {
    // Create a pipe
    let (mut reader, writer) = std::os::unix::net::UnixStream::pair().unwrap();
    let raw_fd = reader.as_raw_fd();
    println!("Original FD: {}", raw_fd);

    // Take ownership via from_raw_fd
    {
        let _file = unsafe { File::from_raw_fd(raw_fd) };
        println!("File created from raw fd");
    } // _file drops here, closing raw_fd

    println!("File dropped");

    // Try to use original reader
    let mut buf = [0; 1];
    match reader.read(&mut buf) {
        Ok(_) => println!("Read success (unexpected)"),
        Err(e) => println!("Read failed: {}", e),
    }
}
