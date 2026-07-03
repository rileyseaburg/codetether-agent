//! Build a standalone BMP file from a packed clipboard DIB.
//!
//! `CF_DIB` clipboard data is a `BITMAPINFOHEADER` (or larger) followed by
//! optional colour masks / palette and the pixel bits — i.e. a `.bmp` file
//! *without* its 14-byte `BITMAPFILEHEADER`. Prepending a correct file
//! header yields a buffer the `image` crate can decode directly.

const FILE_HEADER_LEN: usize = 14;
const BI_BITFIELDS: u32 = 3;

/// Wrap a packed DIB in a `BITMAPFILEHEADER`, producing a full BMP file.
#[cfg_attr(not(windows), allow(dead_code))]
pub(super) fn dib_to_bmp(dib: &[u8]) -> Option<Vec<u8>> {
    let off_bits = FILE_HEADER_LEN + pixel_offset_within_dib(dib)?;
    let file_size = FILE_HEADER_LEN + dib.len();
    let mut bmp = Vec::with_capacity(file_size);
    bmp.extend_from_slice(b"BM");
    bmp.extend_from_slice(&(file_size as u32).to_le_bytes());
    bmp.extend_from_slice(&0u16.to_le_bytes()); // reserved1
    bmp.extend_from_slice(&0u16.to_le_bytes()); // reserved2
    bmp.extend_from_slice(&(off_bits as u32).to_le_bytes());
    bmp.extend_from_slice(dib);
    Some(bmp)
}

/// Byte offset of the pixel data measured from the start of the DIB.
fn pixel_offset_within_dib(dib: &[u8]) -> Option<usize> {
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

fn read_u16(buf: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_le_bytes(buf.get(at..at + 2)?.try_into().ok()?))
}

fn read_u32(buf: &[u8], at: usize) -> Option<u32> {
    Some(u32::from_le_bytes(buf.get(at..at + 4)?.try_into().ok()?))
}
