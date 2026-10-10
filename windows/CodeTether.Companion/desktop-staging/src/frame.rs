//! Memory-only encoded capture with deliberately redacted debug output.
use zeroize::Zeroizing;

/// Bounded JPEG and its encoded dimensions; owned bytes are cleared on drop.
/// No timestamp or trigger is invented here; the scheduling/transport owner
/// supplies protocol metadata and rechecks consent before sending.
///
/// # Examples
/// Constructed only by [`crate::capture_selected`]; illustrative usage:
/// ```text
/// let frame = capture_selected(&selected, &cancelled)?;
/// // frame.width(), frame.height(), frame.jpeg() are upload inputs.
/// ```
pub struct CapturedFrame {
    pub(crate) jpeg: Zeroizing<Vec<u8>>,
    pub(crate) width: u32,
    pub(crate) height: u32,
}
impl CapturedFrame {
    /// Borrow the JPEG for immediate authenticated upload, without a copy.
    pub fn jpeg(&self) -> &[u8] {
        &self.jpeg
    }
    /// Encoded width in physical pixels.
    pub fn width(&self) -> u32 {
        self.width
    }
    /// Encoded height in physical pixels.
    pub fn height(&self) -> u32 {
        self.height
    }
}
impl std::fmt::Debug for CapturedFrame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CapturedFrame")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("jpeg", &"[redacted]")
            .finish()
    }
}
