use nom::combinator::map;
use nom::number::le_u16;
use nom::{IResult, Parser};

use crate::gif::descriptor::LogicalScreenDescriptor;

use super::common::byte;
use super::common::packed_byte;

pub(super) fn logical_screen_descriptor(input: &[u8]) -> IResult<&[u8], LogicalScreenDescriptor> {
    map(
        (
            canvas_width,
            canvas_height,
            packed_byte,
            background_color_idx,
            pixel_aspect_ratio,
        ),
        LogicalScreenDescriptor::from_tuple,
    )
    .parse(input)
}

fn canvas_width(input: &[u8]) -> IResult<&[u8], u16> {
    le_u16().parse(input)
}

fn canvas_height(input: &[u8]) -> IResult<&[u8], u16> {
    le_u16().parse(input)
}

fn background_color_idx(input: &[u8]) -> IResult<&[u8], u8> {
    byte(input)
}

fn pixel_aspect_ratio(input: &[u8]) -> IResult<&[u8], u8> {
    byte(input)
}
