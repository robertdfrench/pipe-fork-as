// Largely derived from https://github.com/nix-rust/nix/blob/414cc86c0af09fd44454b93b6dc738316b16c43c/src/unistd.rs
// Copyright (c) 2015 Carl Lerche + nix-rust Authors
use libc;

use std::mem;
use std::os::unix::io::RawFd;

pub fn pipe() -> (RawFd, RawFd) {
    unsafe {
        let mut fds: [libc::c_int; 2] = mem::uninitialized();
        let res = libc::pipe(fds.as_mut_ptr());
	if res == -1 { panic!("Could not create pipe"); }
        (fds[0], fds[1])
    }
}

pub fn close(fd: RawFd) {
    let res = unsafe { libc::close(fd) };
    if res == -1 { panic!("Could not close file descriptor"); }
}

pub enum ForkResult {
    Parent,
    Child
}

pub fn fork() -> ForkResult {
    let res = unsafe { libc::fork() };
    if res == -1 { panic!("Could not fork"); }
    match res {
        0 => ForkResult::Child,
        _ => ForkResult::Parent
    }
}

pub fn read(fd: RawFd, buf: &mut [u8]) -> usize {
    let res = unsafe {
        libc::read(
            fd,
            buf.as_ptr() as *mut libc::c_void,
            buf.len() as libc::size_t
        )
    };
    if res == -1 { panic!("Could not read from descriptor"); }
    res as usize
}

pub fn write(fd: RawFd, buf: &[u8]) -> usize {
    let res = unsafe {
        libc::write(
            fd,
            buf.as_ptr() as *const libc::c_void,
            buf.len() as libc::size_t
        )
    };
    if res == -1 { panic!("Could not write to descriptor"); }
    res as usize
}
