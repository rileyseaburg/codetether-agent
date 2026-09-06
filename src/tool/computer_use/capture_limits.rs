//! Checked screen-buffer sizing and bounded image transport budgets.
//!
//! Native captures retain their original geometry; excessive allocations are
//! rejected before GDI resources are created, not after memory is exhausted.

pub(crate) const MAX_PIXELS: u64 = 32 * 1024 * 1024;
pub(crate) const MAX_PREVIEW_EDGE: u32 = 2048;
pub(crate) const MAX_PREVIEW_BYTES: usize = 2 * 1024 * 1024;
pub(crate) const MAX_FILE_BYTES: u64 = 64 * 1024 * 1024;

pub(crate) fn pixel_bytes(width: i64, height: i64) -> anyhow::Result<usize> {
    anyhow::ensure!(width > 0 && height > 0, "Capture dimensions must be positive");
    anyhow::ensure!(width <= i32::MAX as i64 && height <= i32::MAX as i64,
        "Capture dimensions exceed native coordinate range");
    let pixels = (width as u64).checked_mul(height as u64)
        .ok_or_else(|| anyhow::anyhow!("Capture dimensions overflow"))?;
    anyhow::ensure!(pixels <= MAX_PIXELS,
        "Capture {width}x{height} exceeds the {MAX_PIXELS}-pixel safety budget; capture a smaller window");
    usize::try_from(pixels.checked_mul(4).ok_or_else(|| anyhow::anyhow!("Capture byte count overflow"))?)
        .map_err(Into::into)
}

pub(crate) fn file_bytes(length: u64) -> anyhow::Result<usize> {
    anyhow::ensure!(length > 0 && length <= MAX_FILE_BYTES, "OCR file is empty or exceeds input safety budget");
    usize::try_from(length).map_err(Into::into)
}

#[cfg(test)]
mod tests {
    #[test]
    fn capture_sizes_are_bounded_before_allocation() {
        assert!(super::file_bytes(0).is_err());
        assert!(super::file_bytes(super::MAX_FILE_BYTES + 1).is_err());
        assert_eq!(super::file_bytes(1024).unwrap(), 1024);
        assert_eq!(super::pixel_bytes(3840, 2160).unwrap(), 33_177_600);
        for (width, height) in [(0, 10), (-1, 20), (i64::MAX, 2), (100_000,100_000)] {
            assert!(super::pixel_bytes(width, height).is_err());
        }
    }
}