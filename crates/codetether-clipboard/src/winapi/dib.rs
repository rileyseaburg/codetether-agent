//! Wrap clipboard DIB pixels with a BMP file header.
//! Build a standalone BMP file from a packed clipboard DIB.
//!
//! `CF_DIB` clipboard data is a `BITMAPINFOHEADER` (or larger) followed by
//! optional colour masks / palette and the pixel bits — i.e. a `.bmp` file
//! *without* its 14-byte `BITMAPFILEHEADER`. Prepending a correct file
//! header yields a buffer the `image` crate can decode directly.
use super::dib_layout::pixel_offset_within_dib;

const FILE_HEADER_LEN: usize = 14;

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
