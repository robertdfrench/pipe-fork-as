pub mod pipe;
pub mod nix;

use std::env;

fn business_logic() -> String {
    "eggman".to_string()
}

fn main() {
    let p = pipe::new();
    let username = env::args().nth(1).unwrap();

    match nix::fork() {
        nix::ForkResult::Child => {
            let uid = nix::getpwnam(&username);
            nix::setuid(uid);

            let results = business_logic();

            p.write(&results);
        },
        nix::ForkResult::Parent => {
            println!("The result from the unprivileged user is: {}", p.read())
        }
    }
}
