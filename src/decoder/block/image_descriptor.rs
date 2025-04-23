use nom::{IResult, Parser, combinator::map, number::complete::le_u16, sequence::preceded};

use crate::{
    decoder::common::{constant, packed_byte},
    gif::descriptor::ImageDescriptor,
};

pub fn image_descriptor(input: &[u8]) -> IResult<&[u8], ImageDescriptor> {
    map(
        preceded(
            ImageSeparator,
            (
                image_left,
                image_top,
                image_width,
                image_height,
                packed_byte,
            ),
        ),
        ImageDescriptor::from_tuple,
    )
    .parse(input)
}

fn image_left(input: &[u8]) -> IResult<&[u8], u16> {
    le_u16(input)
}

fn image_top(input: &[u8]) -> IResult<&[u8], u16> {
    le_u16(input)
}

fn image_width(input: &[u8]) -> IResult<&[u8], u16> {
    le_u16(input)
}

fn image_height(input: &[u8]) -> IResult<&[u8], u16> {
    le_u16(input)
}

constant!(ImageSeparator = 0x2C);
