//! Prefer native PNG, then modern and legacy bitmap clipboard formats.
use super::{dib::dib_to_bmp, image_encode::encode_png_attachment};
use codetether_image::ImageAttachment;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Format {
    Png,
    DibV5,
    Dib,
}

pub(super) fn capture(mut read: impl FnMut(Format) -> Option<Vec<u8>>) -> Option<ImageAttachment> {
    [Format::Png, Format::DibV5, Format::Dib]
        .into_iter()
        .find_map(|format| {
            let bytes = read(format)?;
            match format {
                Format::Png => encode_png_attachment(&bytes, image::ImageFormat::Png),
                Format::DibV5 | Format::Dib => {
                    encode_png_attachment(&dib_to_bmp(&bytes)?, image::ImageFormat::Bmp)
                }
            }
        })
}
