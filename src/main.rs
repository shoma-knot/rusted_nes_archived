extern crate lazy_static;

mod nes;
use nes::Nes;

fn main() {
    let mut nes: Nes = Nes::new();
    nes.fetch("sample1/sample1.nes");
    nes.run();
    //println!("Hello, World!");
}
