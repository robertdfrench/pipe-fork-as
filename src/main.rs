pub mod pipe;
pub mod nix;

fn main() {
    let p = pipe::new();

    match nix::fork() {
        nix::ForkResult::Child => {
            p.write("eggman");
        },
        nix::ForkResult::Parent => {
            println!("The child says {}", p.read())
        }
    }
}
