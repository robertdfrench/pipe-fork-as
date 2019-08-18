pub mod pipe;
pub mod nix;

use std::env;

fn main() {
    let p = pipe::new();
    let username = env::args().nth(1).unwrap();

    match nix::fork() {
        nix::ForkResult::Child => {
            let uid = nix::getpwnam(&username);
            nix::setuid(uid);
            p.write("eggman");
        },
        nix::ForkResult::Parent => {
            println!("The child says {}", p.read())
        }
    }
}
