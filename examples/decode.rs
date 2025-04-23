use std::fs::File;

use giflar::decoder::parse;

fn main() {
    let file = File::open("images/earth.gif").unwrap();

    let gif = parse(file).unwrap();
    let flags = &gif.lsd.flags;

    println!("Parsed: {:#}", gif);
    println!();
    println!("Raw flags: {:08b}", flags.raw);
}
