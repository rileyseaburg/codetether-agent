//! Native clipboard encodings with known pixels, independent of a desktop.
use base64::{Engine as _, engine::general_purpose::STANDARD};
use codetether_image::ImageAttachment;
use image::{DynamicImage, ImageFormat, RgbaImage};
use std::io::Cursor;

pub(super) fn png() -> Vec<u8> {
    let pixels = RgbaImage::from_raw(2, 1, vec![16, 32, 48, 128, 64, 80, 96, 255]).unwrap();
    let mut bytes = Vec::new();
    DynamicImage::ImageRgba8(pixels)
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .unwrap();
    bytes
}

pub(super) fn dib() -> Vec<u8> {
    let pixels = image::load_from_memory(&png()).unwrap().to_rgb8();
    let mut bytes = Vec::new();
    DynamicImage::ImageRgb8(pixels)
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Bmp)
        .unwrap();
    bytes[14..].to_vec()
}

pub(super) fn dib_v5() -> Vec<u8> {
    let mut bytes = vec![0; 124];
    for (offset, value) in [
        (0, 124u32),
        (4, 2),
        (8, 1),
        (16, 3),
        (20, 8),
        (40, 0x00ff0000),
        (44, 0x0000ff00),
        (48, 0x000000ff),
        (52, 0xff000000),
        (56, 0x73524742),
    ] {
        bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    }
    bytes[12..14].copy_from_slice(&1u16.to_le_bytes());
    bytes[14..16].copy_from_slice(&32u16.to_le_bytes());
    bytes.extend_from_slice(&[48, 32, 16, 255, 96, 80, 64, 255]);
    bytes
}

pub(super) fn decoded(attachment: ImageAttachment) -> DynamicImage {
    assert_eq!(attachment.mime_type.as_deref(), Some("image/png"));
    let data = attachment
        .data_url
        .strip_prefix("data:image/png;base64,")
        .unwrap();
    image::load_from_memory_with_format(&STANDARD.decode(data).unwrap(), ImageFormat::Png).unwrap()
}
