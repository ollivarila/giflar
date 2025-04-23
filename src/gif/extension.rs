use super::SubBlock;

#[derive(Debug, Clone, PartialEq)]
pub struct GraphicControlExtension {
    block_size: u8,
    packed_field: u8,
    delay_time: u16,
    transparent_color_idx: u8,
}

impl GraphicControlExtension {
    pub fn from_tuple(
        (block_size, packed_field, delay_time, transparent_color_idx): (u8, u8, u16, u8),
    ) -> Self {
        Self {
            block_size,
            packed_field,
            delay_time,
            transparent_color_idx,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ApplicationExtension {
    NetScape(NetScapeExtension),
}

#[derive(Debug, Clone, PartialEq)]
pub struct NetScapeExtension {
    pub sub_blocks: Vec<SubBlock>,
}

impl NetScapeExtension {
    pub fn new(sub_blocks: Vec<SubBlock>) -> Self {
        Self { sub_blocks }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommentExtension;

#[derive(Debug, Clone, PartialEq)]
pub struct PlainTextExtension;
