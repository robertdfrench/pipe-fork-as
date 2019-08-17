pub mod pipe;
pub mod nix;

fn main() {
    let _p = pipe::new();

    match nix::fork() {
        nix::ForkResult::Child => println!("Hello from child"),
        nix::ForkResult::Parent => println!("Hello from parent")
    }

    println!("Hello, world!");
}
