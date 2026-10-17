//! Checked native pixel-buffer allocation bounds.
//!
//! Native captures retain their original geometry; excessive allocations are
//! rejected before GDI resources are created, not after memory is exhausted.

/// Maximum number of pixels accepted in one native capture buffer.
pub const MAX_PIXELS: u64 = 32 * 1024 * 1024;
/// Compute a checked four-byte-per-pixel allocation size.
///
/// # Arguments
/// * `width` - Positive width within native signed-coordinate range.
/// * `height` - Positive height within native signed-coordinate range.
///
/// # Returns
/// The byte count for the original capture geometry.
///
/// # Errors
/// Rejects nonpositive or oversized dimensions, excess pixels, and arithmetic
/// or address-space overflow before a buffer is allocated.
///
/// # Examples
/// ```
/// use codetether_platform::capture::pixel_bytes;
/// assert_eq!(pixel_bytes(1920, 1080).unwrap(), 8_294_400);
/// assert!(pixel_bytes(0, 1080).is_err());
/// ```
pub fn pixel_bytes(width: i64, height: i64) -> anyhow::Result<usize> {
    anyhow::ensure!(
        width > 0 && height > 0,
        "Capture dimensions must be positive"
    );
    anyhow::ensure!(
        width <= i32::MAX as i64 && height <= i32::MAX as i64,
        "Capture dimensions exceed native coordinate range"
    );
    let pixels = (width as u64)
        .checked_mul(height as u64)
        .ok_or_else(|| anyhow::anyhow!("Capture dimensions overflow"))?;
    anyhow::ensure!(
        pixels <= MAX_PIXELS,
        "Capture {width}x{height} exceeds the {MAX_PIXELS}-pixel safety budget; capture a smaller window"
    );
    usize::try_from(
        pixels
            .checked_mul(4)
            .ok_or_else(|| anyhow::anyhow!("Capture byte count overflow"))?,
    )
    .map_err(Into::into)
}
