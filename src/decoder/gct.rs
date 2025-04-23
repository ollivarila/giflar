use super::common::byte;
use crate::gif::table::GlobalColorTable;
use crate::gif::{Color, table::LocalColorTable};
use nom::{IResult, Parser, combinator::map, multi::many};

pub fn global_color_table(input: &[u8], len: usize) -> IResult<&[u8], GlobalColorTable> {
    map(many(0..=len, color), GlobalColorTable::new).parse(input)
}

pub fn local_color_table(input: &[u8], len: usize) -> IResult<&[u8], LocalColorTable> {
    map(many(0..=len, color), GlobalColorTable::new).parse(input)
}

pub fn color(input: &[u8]) -> IResult<&[u8], Color> {
    map((byte, byte, byte), Color::from_triple).parse(input)
}
