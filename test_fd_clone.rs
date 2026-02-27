
use std::os::fd::{RawFd, BorrowedFd, AsRawFd};
use std::fs::File;

fn main() {
    // Create a pipe to get a valid FD
    let (reader, writer) = std::os::unix::net::UnixStream::pair().unwrap();
    let raw_fd = reader.as_raw_fd();

    // Mimic the logic
    let borrowed = unsafe { BorrowedFd::borrow_raw(raw_fd) };
    let owned = borrowed.try_clone_to_owned().unwrap();
    let file: File = owned.into();

    println!("Original FD: {}, Cloned FD: {}", raw_fd, file.as_raw_fd());
}
