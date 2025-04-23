use nom::{
    IResult, Parser,
    bytes::complete::{tag, take},
    combinator::{map, value},
    number::le_u16,
    sequence::{delimited, preceded},
};

use super::{block_len, sub_blocks};
use crate::{
    decoder::common::{byte, constant, packed_byte as packed_field},
    gif::extension::{
        ApplicationExtension, CommentExtension, GraphicControlExtension, NetScapeExtension,
        PlainTextExtension,
    },
};

use super::BlockTerminator;

constant!(GraphicControlLabel = 0xF9);
constant!(ExtensionIntroducer = 0x21);
constant!(PlainTextLabel = 0x01);
constant!(ApplicationExtensionLabel = 0xFF);
constant!(CommentExtensionLabel = 0xFE);

pub fn graphic_control_extension(input: &[u8]) -> IResult<&[u8], GraphicControlExtension> {
    let start_signature = (ExtensionIntroducer, GraphicControlLabel);

    map(
        delimited(
            start_signature,
            (byte_size, packed_field, delay_time, transparent_color_idx),
            BlockTerminator,
        ),
        GraphicControlExtension::from_tuple,
    )
    .parse(input)
}

fn byte_size(input: &[u8]) -> IResult<&[u8], u8> {
    byte(input)
}

fn delay_time(input: &[u8]) -> IResult<&[u8], u16> {
    le_u16().parse(input)
}

fn transparent_color_idx(input: &[u8]) -> IResult<&[u8], u8> {
    byte(input)
}

pub fn plain_text_extension(input: &[u8]) -> IResult<&[u8], PlainTextExtension> {
    let start_signature = (ExtensionIntroducer, PlainTextLabel);
    let (rest, len) = preceded(start_signature, block_len).parse(input)?;
    let chomp = take(len);

    value(PlainTextExtension, (chomp, sub_blocks)).parse(rest)
}

pub fn comment_extension(input: &[u8]) -> IResult<&[u8], CommentExtension> {
    let start_signature = (ExtensionIntroducer, CommentExtensionLabel);
    value(CommentExtension, (start_signature, sub_blocks)).parse(input)
}

/// Only NETSCAPE2.0 is recognized
pub fn application_extension(input: &[u8]) -> IResult<&[u8], ApplicationExtension> {
    let netscape = b"NETSCAPE2.0";
    let start_signature = (
        ExtensionIntroducer,
        ApplicationExtensionLabel,
        tag([0x0B].as_ref()),
        tag(netscape.as_ref()),
    );

    preceded(start_signature, sub_blocks)
        .map(NetScapeExtension::new)
        .map(ApplicationExtension::NetScape)
        .parse(input)
}

#[cfg(test)]
mod should {
    use super::*;

    #[test]
    fn parse_application_ext() {
        let data = b"\x21\xFF\x0BNETSCAPE2.0\x03\x01\x00\x00\x00";

        let (rem, app_ext) = application_extension(data).unwrap();

        let ApplicationExtension::NetScape(netscape) = app_ext;

        assert!(rem.is_empty());
        assert_eq!(netscape.sub_blocks.len(), 1);
    }
}
