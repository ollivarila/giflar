#[derive(Debug, Clone, PartialEq)]
pub struct LogicalScreenDescriptor {
    pub canvas_width: u16,
    pub canvas_height: u16,
    pub flags: Flags,
    /// Should be set if image has global color table
    pub background_color_idx: u8,
    pub pixel_aspect_ratio: u8,
}

impl LogicalScreenDescriptor {
    pub fn from_tuple(
        (canvas_width, canvas_height, packed_byte, background_color_idx, pixel_aspect_ratio): (
            u16,
            u16,
            u8,
            u8,
            u8,
        ),
    ) -> Self {
        Self {
            canvas_width,
            canvas_height,
            flags: packed_byte.into(),
            background_color_idx,
            pixel_aspect_ratio,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Flags {
    pub raw: u8,
    pub global_color_table: bool,
    pub color_resolution: u8,
    pub sort: bool,
    /// Actual size computed as 2 ^ (n + 1)
    color_table_size: u8,
}

impl Flags {
    const GLOBAL_COLOR_TABLE_MASK: u8 = 0b1000_0000;
    const COLOR_RES_MASK: u8 = 0b0111_0000;
    const SORT_MASK: u8 = 0b0000_1000;
    const COLOR_TABLE_SIZE_MASK: u8 = 0b0000_0111;

    /// Returns 1-256
    pub fn color_table_size(&self) -> u32 {
        let exp = self.color_table_size + 1;
        2u32.pow(exp.into())
    }
}

impl From<u8> for Flags {
    fn from(value: u8) -> Self {
        Self {
            raw: value,
            global_color_table: (value & Flags::GLOBAL_COLOR_TABLE_MASK) != 0,
            color_resolution: ((value & Flags::COLOR_RES_MASK) >> 4) + 1,
            sort: (value & Flags::SORT_MASK) != 0,
            color_table_size: (value & Flags::COLOR_TABLE_SIZE_MASK),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImageDescriptor {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub packed_byte: u8,
}

impl ImageDescriptor {
    const LOCAL_COLOR_TABLE_MASK: u8 = 0b1000_0000;

    pub fn has_local_color_table(&self) -> bool {
        (self.packed_byte & ImageDescriptor::LOCAL_COLOR_TABLE_MASK) != 0
    }

    const COLOR_TABLE_SIZE_MASK: u8 = 0b0000_0111;

    pub fn color_table_size(&self) -> u32 {
        let exp = (self.packed_byte & ImageDescriptor::COLOR_TABLE_SIZE_MASK) + 1;
        2u32.pow(exp.into())
    }
}

impl ImageDescriptor {
    pub fn from_tuple((left, top, width, height, packed_byte): (u16, u16, u16, u16, u8)) -> Self {
        Self {
            left,
            top,
            width,
            height,
            packed_byte,
        }
    }
}
