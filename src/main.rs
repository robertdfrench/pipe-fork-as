pub mod pipe;
pub mod nix;

fn main() {
    let _p = pipe::Pipe::new();
    println!("Hello, world!");
}
