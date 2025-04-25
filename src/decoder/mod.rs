use std::borrow::Cow;
use std::fmt::Debug;
use std::io::Read;

use block::blocks;
use lzw::lzw_decode;
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
mod lzw;

use gct::global_color_table;
use lsd::logical_screen_descriptor;

use crate::gif::{Block, Frame, Gif, GifVersion, ImageBlock, ImageContent};

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

pub fn decode(gif: Gif) -> Result<Vec<Frame>, DecodeError> {
    GifDecoder::create(gif).decode()
}

#[derive(Clone)]
struct GifDecoder {
    gif: Gif,
    decoded_frames: Vec<Frame>,
}

impl GifDecoder {
    fn create(gif: Gif) -> Self {
        Self {
            decoded_frames: Vec::with_capacity(gif.blocks.len()),
            gif,
        }
    }

    fn decode(mut self) -> Result<Vec<Frame>, DecodeError> {
        for block in &self.gif.blocks {
            if let Some(frame) = self.decode_frame(block)? {
                self.decoded_frames.push(frame);
            };
        }

        Ok(self.decoded_frames)
    }

    fn decode_frame(&self, block: &Block) -> Result<Option<Frame>, DecodeError> {
        match block {
            Block::CommentExtension(_) | Block::ApplicationExtension(_) => Ok(None),
            Block::Image(image_block) => self.decode_image_block(image_block),
        }
    }

    fn decode_image_block(&self, block: &ImageBlock) -> Result<Option<Frame>, DecodeError> {
        match &block.content {
            ImageContent::PlainText(_) => Ok(None),
            ImageContent::Image(image) => {
                let color_table = image
                    .lct
                    .as_ref()
                    .or(self.gif.gct.as_ref())
                    .expect("one color table should be defined");

                lzw_decode(&image.descriptor, &image.data, color_table).map(Some)
            }
        }
    }
}
