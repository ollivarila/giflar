use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub enum GifVersion {
    V89a,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trailer;

#[derive(Debug, Clone, PartialEq)]
pub struct ImageData {
    pub lzw_min_code_size: u8,
    pub sub_blocks: Vec<SubBlock>,
}

impl ImageData {
    pub fn from_tuple((lzw_min_code_size, sub_blocks): (u8, Vec<SubBlock>)) -> Self {
        Self {
            lzw_min_code_size,
            sub_blocks,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SubBlock(pub Vec<u8>);

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn as_hex(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    pub fn from_triple((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_hex())
    }
}
