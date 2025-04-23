use extension::{
    application_extension, comment_extension, graphic_control_extension, plain_text_extension,
};
use image_descriptor::image_descriptor;
use nom::{
    IResult, Parser,
    branch::alt,
    combinator::{cond, opt},
    multi::{many, many_till},
    sequence::pair,
};

use crate::gif::{Block, ImageContent, ImageData, SubBlock};

use super::{
    common::{byte, constant},
    gct::local_color_table,
};

mod extension;
mod image_descriptor;

constant!(BlockTerminator = 0x00);
constant!(Trailer = 0x3B);

pub fn blocks(input: &[u8]) -> IResult<&[u8], Vec<Block>> {
    many_till(block, Trailer)
        .map(|(blocks, _)| blocks)
        .parse(input)
}

fn block(input: &[u8]) -> IResult<&[u8], Block> {
    alt((
        image_block,
        application_extension_block,
        comment_extension_block,
    ))
    .parse(input)
}

fn image_block(input: &[u8]) -> IResult<&[u8], Block> {
    (opt(graphic_control_extension), image_content)
        .map(Block::image_block_from_tuple)
        .parse(input)
}

fn image_content(input: &[u8]) -> IResult<&[u8], ImageContent> {
    alt((pixel_content, plain_text_content)).parse(input)
}

fn pixel_content(input: &[u8]) -> IResult<&[u8], ImageContent> {
    let (rest, descriptor) = image_descriptor(input)?;
    let has_lct = descriptor.has_local_color_table();
    let len = descriptor.color_table_size() as usize;

    let (rest, (lct, data)) = pair(
        cond(has_lct, |input| local_color_table(input, len)),
        image_data,
    )
    .parse(rest)?;

    let image_content = ImageContent::image_from_tuple((descriptor, lct, data));

    Ok((rest, image_content))
}

fn plain_text_content(input: &[u8]) -> IResult<&[u8], ImageContent> {
    plain_text_extension
        .map(ImageContent::PlainText)
        .parse(input)
}

fn image_data(input: &[u8]) -> IResult<&[u8], ImageData> {
    let lzw_min = byte;
    (lzw_min, sub_blocks)
        .map(ImageData::from_tuple)
        .parse(input)
}

fn sub_blocks(input: &[u8]) -> IResult<&[u8], Vec<SubBlock>> {
    many_till(sub_block, BlockTerminator)
        .map(|(blocks, _)| blocks)
        .parse(input)
}

fn sub_block(input: &[u8]) -> IResult<&[u8], SubBlock> {
    let (rest, len) = block_len(input)?;
    many(0..=len as usize, byte).map(SubBlock).parse(rest)
}

fn block_len(input: &[u8]) -> IResult<&[u8], u8> {
    byte(input)
}

fn application_extension_block(input: &[u8]) -> IResult<&[u8], Block> {
    application_extension
        .map(Block::ApplicationExtension)
        .parse(input)
}

fn comment_extension_block(input: &[u8]) -> IResult<&[u8], Block> {
    comment_extension.map(Block::CommentExtension).parse(input)
}

#[cfg(test)]
mod should {}
