//! Locate DIB pixels after color masks and palettes.
use super::dib_bytes::{read_u16, read_u32};

const BI_BITFIELDS: u32 = 3;

/// Byte offset of the pixel data measured from the start of the DIB.
pub(super) fn pixel_offset_within_dib(dib: &[u8]) -> Option<usize> {
    let header_size = read_u32(dib, 0)? as usize;
    let bit_count = read_u16(dib, 14)? as usize;
    let compression = read_u32(dib, 16)?;
    let clr_used = read_u32(dib, 32)? as usize;
    let masks = if header_size == 40 && compression == BI_BITFIELDS {
        12 // three DWORD colour masks follow a bare BITMAPINFOHEADER
    } else {
        0
    };
    Some(header_size + masks + palette_bytes(bit_count, clr_used))
}

/// Size in bytes of the colour palette that follows the header.
fn palette_bytes(bit_count: usize, clr_used: usize) -> usize {
    let entries = match (bit_count, clr_used) {
        (bc, 0) if bc <= 8 => 1usize << bc,
        (_, used) => used,
    };
    entries * 4
}
