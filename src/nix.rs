use libc;

use std::mem;
use std::os::unix::io::RawFd;

/// Create a pair of file descriptors for a unix pipe
///
/// Derived from:
/// https://github.com/nix-rust/nix/blob/414cc86c0af09fd44454b93b6dc738316b16c43c/src/unistd.rs#L1008
pub fn pipe() -> (RawFd, RawFd) {
    unsafe {
        let mut fds: [libc::c_int; 2] = mem::uninitialized();
        let res = libc::pipe(fds.as_mut_ptr());
	if res == -1 { panic!("Could not create pipe"); }
        (fds[0], fds[1])
    }
}

/// Close a raw file descriptor
///
/// Derived from:
/// https://github.com/nix-rust/nix/blob/414cc86c0af09fd44454b93b6dc738316b16c43c/src/unistd.rs#L933
pub fn close(fd: RawFd) {
    let res = unsafe { libc::close(fd) };
    if res == -1 { panic!("Could not close file descriptor"); }
}
