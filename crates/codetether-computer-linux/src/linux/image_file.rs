//! Validate file and decoded pixel budgets before attaching any screenshot.

use codetether_computer_capture::limits::{MAX_FILE_BYTES, file_bytes, pixel_bytes};
use image::{ImageFormat, ImageReader};
use std::{
    io::{Cursor, Read},
    path::Path,
};

pub(super) fn load(path: &Path) -> anyhow::Result<(Vec<u8>, u32, u32)> {
    let file = std::fs::File::open(path)?;
    file_bytes(file.metadata()?.len())?;
    let mut png = Vec::new();
    file.take(MAX_FILE_BYTES + 1).read_to_end(&mut png)?;
    file_bytes(png.len() as u64)?;
    anyhow::ensure!(
        png.starts_with(b"\x89PNG\r\n\x1a\n"),
        "Screenshot utility did not produce PNG data"
    );
    let (width, height) =
        ImageReader::with_format(Cursor::new(&png), ImageFormat::Png).into_dimensions()?;
    pixel_bytes(i64::from(width), i64::from(height))?;
    let mut reader = ImageReader::with_format(Cursor::new(&png), ImageFormat::Png);
    let mut limits = image::Limits::default();
    limits.max_alloc = Some(128 * 1024 * 1024);
    reader.limits(limits);
    reader.decode()?;
    Ok((png, width, height))
}
