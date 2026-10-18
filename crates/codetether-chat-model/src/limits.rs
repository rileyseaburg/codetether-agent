//! Retained text limits for live frontend previews.
//!
//! Frontends apply this bound to cumulative assistant text while streaming.
/// Maximum retained streaming preview size, in bytes (256 KiB).
pub const MAX_STREAMING_TEXT_BYTES: usize = 256 * 1024;
