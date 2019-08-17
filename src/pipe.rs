use std::os::unix::io::RawFd;
use crate::nix;

pub struct Pipe {
    input: RawFd,
    output: RawFd
}

pub fn new() -> Pipe {
    let (in_fd, out_fd) = nix::pipe();
    Pipe {
        input: in_fd,
        output: out_fd
    }
}

impl Drop for Pipe {
    fn drop(&mut self) {
        nix::close(self.input);
        nix::close(self.output);
    }
}
