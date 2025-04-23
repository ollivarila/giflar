use std::fs::File;

use giflar::decoder::decode;

fn main() {
    let file = File::open("images/earth.gif").unwrap();

    let gif = decode(file).unwrap();
    let flags = &gif.lsd.flags;

    println!("Decoded: {:#}", gif);
    println!();
    println!("Raw flags: {:08b}", flags.raw);
}
