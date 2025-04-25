use crate::gif::{Frame, ImageData, SubBlock, descriptor::ImageDescriptor, table::ColorTable};

use super::DecodeError;

pub fn lzw_decode(
    descriptor: &ImageDescriptor,
    data: &ImageData,
    color_table: &ColorTable,
) -> Result<Frame, DecodeError> {
    let mut code_size = data.lzw_min_code_size as u16;
    let mut code_stream = CodeStream::new(data);
    let first_code = code_stream.next_code(code_size);
    todo!()
}

/// Transforms sub blocks into a continuous stream of bytes
struct ByteStream<'a> {
    source: &'a Vec<SubBlock>,
    source_idx: usize,
    block_idx: usize,
}

impl<'a> ByteStream<'a> {
    fn new(blocks: &'a Vec<SubBlock>) -> ByteStream<'a> {
        assert!(!blocks.is_empty());
        Self {
            source: blocks,
            block_idx: 0,
            source_idx: 0,
        }
    }

    fn at_end(&self) -> bool {
        if self.source_idx < self.source.len() - 1 {
            return false;
        }

        let block = &self.source[self.source_idx];

        self.block_idx < block.0.len()
    }
}

impl Iterator for ByteStream<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.at_end() {
            return None;
        }

        self.block_idx += 1;

        let block = &self.source[self.source_idx];

        if let Some(byte) = block.0.get(self.block_idx) {
            return Some(*byte);
        } else {
            self.source_idx += 1;
            self.next()
        }
    }
}

/// Extracts LZW codes from bytes
struct CodeStream<'a> {
    bbuf: BitBuffer,
    bytes: ByteStream<'a>,
}

impl<'a> CodeStream<'a> {
    fn new(data: &'a ImageData) -> CodeStream<'a> {
        Self {
            bbuf: BitBuffer::default(),
            bytes: ByteStream::new(&data.sub_blocks),
        }
    }

    fn next_code(&mut self, code_size: u16) -> u16 {
        assert!(code_size <= 4095);
        self.bbuf.fill(&mut self.bytes);
        self.bbuf.pop_front(code_size).expect("code")
    }
}

#[derive(Default)]
struct BitBuffer {
    buf: u64,
    n_bits: u8,
}

impl BitBuffer {
    fn fill<It: Iterator<Item = u8>>(&mut self, it: &mut It) {
        while self.has_space_left() {
            if let Some(byte) = it.next() {
                self.push_back(byte);
            } else {
                break;
            }
        }
    }

    fn push_back(&mut self, byte: u8) {
        assert!(self.space_left() > 0);
        let shift = (64 - self.n_bits) as u64 - 8;

        let yep = (byte as u64) << shift;
        self.buf += yep;
        self.n_bits += 8;
    }

    /// Space left n bytes
    fn space_left(&self) -> u8 {
        const MAX: u8 = 8 * 8;
        MAX - self.n_bits
    }

    fn has_space_left(&self) -> bool {
        self.space_left() > 0
    }

    // 1111 0000
    // 1100 0011 & 0000 0111 -> 0000 0011
    //
    // 1100 0011 & 1111 1000 -> 1100 0000
    //
    // 1100 0000 + fill -> 1100 0010 (fill was 10)

    fn pop_front(&mut self, code_size: u16) -> Option<u16> {
        if self.n_bits == 0 || code_size + 1 > self.n_bits.into() {
            return None;
        }

        // move next bits to front
        let rotated = self.buf.rotate_left(code_size as u32 + 1);

        let desired_mask = (1u16 << (code_size + 1)) - 1;
        let rest_mask = desired_mask.reverse_bits();

        // take first code_size bits
        let desired = rotated & desired_mask as u64;
        // remove the popped bits

        let rotated_and_removed = rotated & rest_mask as u64;

        self.buf = rotated_and_removed;
        self.n_bits -= (code_size + 1) as u8;

        assert!(desired <= 4095);
        Some(desired as u16)
    }
}

#[cfg(test)]
mod should {

    use super::*;

    // Bit buffer

    #[test]
    fn push_bytes_to_buf() {
        let mut buf = BitBuffer::default();
        buf.push_back(0xFF);
        // println!("{:0x}", buf.buf);
        assert_eq!(
            buf.buf,
            0b11111111_00000000_00000000_00000000_00000000_00000000_00000000_00000000
        );
        assert_eq!(buf.n_bits, 8);

        buf.push_back(0b00110011);
        assert_eq!(
            buf.buf,
            0b11111111_00110011_00000000_00000000_00000000_00000000_00000000_00000000
        );
        assert_eq!(buf.n_bits, 16);
    }

    #[test]
    fn pop_front_from_buf() {
        let mut buf = BitBuffer::default();
        buf.push_back(0b00110011);
        let code = buf.pop_front(2).unwrap();

        assert_eq!(code, 0b001);
        assert_eq!(buf.n_bits, 5);
        assert!(buf.has_space_left());
        assert_eq!(buf.space_left(), 64 - 5);
    }

    #[test]
    fn fill_buf() {
        let mut buf = BitBuffer::default();
        let bytes: &[u8] = &[0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0x00];
        let mut it = bytes.iter().cloned();
        buf.fill(&mut it);

        assert!(!buf.has_space_left());
        assert_eq!(buf.buf, 0xFF_FF_FF_FF_FF_FF_FF_00)
    }

    #[test]
    fn return_none_when_empty() {
        let mut buf = BitBuffer::default();
        assert_eq!(buf.pop_front(2), None);
    }
}
