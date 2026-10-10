//! Aspect-preserving bilinear BGRA-to-RGB scaling, longest edge at most 1600.
use super::{cancel, dib::Surface, pixels::Pixels};
use crate::{Bounds, Error};
use std::sync::atomic::AtomicBool;

pub(super) fn resize(
    surface: &mut Surface<'_>,
    bounds: Bounds,
    cancelled: &AtomicBool,
) -> Result<Pixels, Error> {
    let longest = bounds.width().max(bounds.height());
    let edge = longest.min(1600);
    let width = (u64::from(bounds.width()) * u64::from(edge) / u64::from(longest)).max(1) as u32;
    let height = (u64::from(bounds.height()) * u64::from(edge) / u64::from(longest)).max(1) as u32;
    let mut output = Pixels::new(width, height);
    let source = surface.bytes();
    for y in 0..height {
        cancel::check(cancelled)?;
        let sy =
            ((f64::from(y) + 0.5) * f64::from(bounds.height()) / f64::from(height) - 0.5).max(0.0);
        let y0 = sy.floor() as u32;
        let y1 = (y0 + 1).min(bounds.height() - 1);
        for x in 0..width {
            let sx = ((f64::from(x) + 0.5) * f64::from(bounds.width()) / f64::from(width) - 0.5)
                .max(0.0);
            let x0 = sx.floor() as u32;
            let x1 = (x0 + 1).min(bounds.width() - 1);
            let fx = sx - f64::from(x0);
            let fy = sy - f64::from(y0);
            for channel in 0..3_usize {
                let sample = |px: u32, py: u32| -> f64 {
                    f64::from(
                        source[(py as usize * bounds.width() as usize + px as usize) * 4 + 2
                            - channel],
                    )
                };
                let top = sample(x0, y0) * (1.0 - fx) + sample(x1, y0) * fx;
                let bottom = sample(x0, y1) * (1.0 - fx) + sample(x1, y1) * fx;
                output.bytes[(y as usize * width as usize + x as usize) * 3 + channel] =
                    (top * (1.0 - fy) + bottom * fy).round().clamp(0.0, 255.0) as u8;
            }
        }
    }
    cancel::check(cancelled)?;
    Ok(output)
}
