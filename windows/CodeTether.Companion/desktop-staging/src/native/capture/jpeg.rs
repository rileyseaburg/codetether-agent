//! Reference quality ladder with a hard encoded-output size boundary.
use super::{cancel, output::Output, pixels::Pixels};
use crate::Error;
use jpeg_encoder::{ColorType, Encoder};
use std::sync::atomic::AtomicBool;
use zeroize::Zeroizing;

pub(super) fn encode(pixels: &Pixels, cancelled: &AtomicBool) -> Result<Zeroizing<Vec<u8>>, Error> {
    let width = u16::try_from(pixels.width).map_err(|_| Error::Encoding)?;
    let height = u16::try_from(pixels.height).map_err(|_| Error::Encoding)?;
    for quality in [75, 55, 35, 20] {
        cancel::check(cancelled)?;
        let mut output = Output::new(cancelled);
        let result =
            Encoder::new(&mut output, quality).encode(&pixels.bytes, width, height, ColorType::Rgb);
        cancel::check(cancelled)?;
        if result.is_ok() {
            return Ok(output.bytes);
        }
        if !output.too_large {
            return Err(Error::Encoding);
        }
    }
    Err(Error::FrameTooLarge)
}
