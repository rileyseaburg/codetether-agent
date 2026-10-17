//! Tool-specific preview and input-file budgets with shared pixel allocation checks.

pub(crate) use codetether_platform::capture::pixel_bytes;
pub(crate) const MAX_PREVIEW_EDGE: u32 = 2048;
pub(crate) const MAX_PREVIEW_BYTES: usize = 2 * 1024 * 1024;
pub(crate) const MAX_FILE_BYTES: u64 = 64 * 1024 * 1024;

pub(crate) fn file_bytes(length: u64) -> anyhow::Result<usize> {
    anyhow::ensure!(
        length > 0 && length <= MAX_FILE_BYTES,
        "OCR file is empty or exceeds input safety budget"
    );
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
        for (width, height) in [(0, 10), (-1, 20), (i64::MAX, 2), (100_000, 100_000)] {
            assert!(super::pixel_bytes(width, height).is_err());
        }
    }
}
