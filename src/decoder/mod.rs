use std::borrow::Cow;
use std::fmt::Debug;
use std::io::Read;

use block::blocks;
use nom::Parser;
use nom::bytes::{tag, take};
use nom::combinator::{all_consuming, cond, map};
use nom::sequence::pair;
use nom::{IResult, sequence::preceded};
use thiserror::Error;

mod block;
mod common;
mod gct;
mod lsd;

use gct::global_color_table;
use lsd::logical_screen_descriptor;

use crate::gif::{Gif, GifVersion};

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Encountered invalid data while decoding GIF: {0}")]
    InvalidData(Cow<'static, str>),
    #[error("Failed to decode GIF")]
    Fail,
    #[error("Io error while decoding GIF `{0}`")]
    Io(#[from] std::io::Error),
}

pub fn parse<R: Read>(mut source: R) -> Result<Gif, DecodeError> {
    let mut bytes = Vec::new();
    source.read_to_end(&mut bytes)?;

    let (rest, version_bytes) = header(&bytes)
        .map_err(|_| DecodeError::InvalidData("failed to parse GIF header".into()))?;

    let version = version_bytes.try_into()?;

    let (rest, logical_screen_descriptor) = logical_screen_descriptor(rest).map_err(|_| {
        DecodeError::InvalidData("failed to parse logical screen descriptor".into())
    })?;

    let has_gct = logical_screen_descriptor.flags.global_color_table;
    let len = logical_screen_descriptor.flags.color_table_size() as usize;

    let (_, (gct, blocks)) = all_consuming(pair(
        cond(has_gct, |input| global_color_table(input, len)),
        blocks,
    ))
    .parse(rest)
    .map_err(|_| DecodeError::InvalidData("failed to parse global color table or blocks".into()))?;

    Ok(Gif {
        version,
        lsd: logical_screen_descriptor,
        gct,
        blocks,
    })
}

impl TryFrom<VersionBytes<'_>> for GifVersion {
    type Error = DecodeError;
    fn try_from(value: VersionBytes) -> Result<Self, Self::Error> {
        match value.0 {
            b"89a" => Ok(GifVersion::V89a),
            _ => Err(DecodeError::InvalidData(
                format!("invalid version bytes: `{:?}` (only 89a supported)", value).into(),
            )),
        }
    }
}

/// May not be a valid version
#[derive(Debug)]
struct VersionBytes<'a>(&'a [u8]);

fn header(input: &[u8]) -> IResult<&[u8], VersionBytes> {
    map(preceded(tag("GIF"), take(3usize)), VersionBytes).parse(input)
}
