
use std::os::fd::FromRawFd;
use std::fs::File;
use std::os::unix::ffi::OsStrExt;

fn main() {
    #[cfg(target_os = "linux")]
    unsafe {
        let name = std::ffi::CString::new("test").unwrap();
        let fd = libc::memfd_create(name.as_ptr(), 0);
        if fd < 0 {
            perror("memfd_create");
            return;
        }
        let path = format!("/proc/self/fd/{}", fd);
        let link = std::fs::read_link(&path).unwrap();
        println!("Link: {:?}", link);

        let path_bytes = link.as_os_str().as_bytes();
        let prefix = b"/memfd:";
        if path_bytes.starts_with(prefix) {
             println!("Starts with /memfd:");
        } else if path_bytes.starts_with(b"memfd:") {
             println!("Starts with memfd:");
        }
    }
}

fn perror(s: &str) {
    let s = std::ffi::CString::new(s).unwrap();
    unsafe { libc::perror(s.as_ptr()) };
}
