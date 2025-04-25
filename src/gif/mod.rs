pub mod data;
pub mod descriptor;
pub mod extension;
pub mod table;

use std::fmt::{Debug, Display, Formatter};

pub use data::*;
use descriptor::{ImageDescriptor, LogicalScreenDescriptor};
use extension::{
    ApplicationExtension, CommentExtension, GraphicControlExtension, PlainTextExtension,
};
use table::{GlobalColorTable, LocalColorTable};

#[derive(Clone)]
pub struct Frame {
    pub pixels: Vec<Color>,
}

/// Represents a parsed GIF
/// https://www.matthewflickinger.com/lab/whatsinagif/bits_and_bytes.asp
#[derive(Clone)]
pub struct Gif {
    pub version: GifVersion,
    pub lsd: LogicalScreenDescriptor,
    pub gct: Option<GlobalColorTable>,
    pub blocks: Vec<Block>,
}

impl Display for Gif {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut d = f.debug_struct("Gif");

        d.field("version", &self.version)
            .field("lsd", &self.lsd)
            .field("gct", &self.gct);

        if self.blocks.len() > 100 {
            d.field("blocks", &self.blocks.len());
        } else {
            d.field("blocks", &self.blocks);
        }

        d.finish()
    }
}

impl Gif {
    pub fn from_tuple(
        (version, lsd, gct, blocks): (
            GifVersion,
            LogicalScreenDescriptor,
            Option<GlobalColorTable>,
            Vec<Block>,
        ),
    ) -> Self {
        Self {
            version,
            lsd,
            gct,
            blocks,
        }
    }
}

#[derive(Clone)]
pub enum Block {
    Image(ImageBlock),
    ApplicationExtension(ApplicationExtension),
    CommentExtension(CommentExtension),
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Block::Image(_) => write!(f, "ImageBlock"),
            Block::ApplicationExtension(_) => write!(f, "ApplicationExtension"),
            Block::CommentExtension(_) => write!(f, "CommentExtension"),
        }
    }
}

impl Block {
    pub fn image_block_from_tuple(
        (gce, content): (Option<GraphicControlExtension>, ImageContent),
    ) -> Self {
        Self::Image(ImageBlock { gce, content })
    }
}

#[derive(Debug, Clone)]
pub struct ImageBlock {
    pub gce: Option<GraphicControlExtension>,
    pub content: ImageContent,
}

#[derive(Debug, Clone)]
pub enum ImageContent {
    Image(Image),
    PlainText(PlainTextExtension),
}

impl ImageContent {
    pub fn plain_text(extension: PlainTextExtension) -> Self {
        ImageContent::PlainText(extension)
    }

    pub fn image_from_tuple(
        (descriptor, lct, data): (ImageDescriptor, Option<LocalColorTable>, ImageData),
    ) -> Self {
        ImageContent::Image(Image {
            descriptor,
            lct,
            data,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    pub descriptor: ImageDescriptor,
    pub lct: Option<LocalColorTable>,
    pub data: ImageData,
}
