use std::os::unix::io::RawFd;
use crate::nix;

pub struct Pipe {
    read_end: RawFd,
    write_end: RawFd
}

pub fn new() -> Pipe {
    let (r, w) = nix::pipe();
    Pipe {
        read_end: r,
        write_end: w
    }
}

impl Pipe {
    pub fn write(&self, message: &str) {
        nix::write(self.write_end, message.as_bytes());
        nix::close(self.write_end);
    }
    pub fn read(&self) -> String {
        let mut buf: [u8; 1024] = [0; 1024];
        nix::read(self.read_end, &mut buf);
        String::from_utf8(buf.to_vec()).unwrap()
    }
}
