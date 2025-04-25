use std::fs::File;

use giflar::decoder::parse;

fn main() {
    let file = File::open("images/earth.gif").unwrap();

    let gif = parse(file).unwrap();

    println!("Parsed: {:#}", gif);
}
