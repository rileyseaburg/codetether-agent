//! Explain why an image data URL was rejected.
//!
//! Without this, an oversized screenshot silently falls through to the
//! large-paste text sidecar and the user sees no image and no error.

use super::data_url::{MAX_BASE64_PAYLOAD_CHARS, MAX_IMAGE_DECODED_BYTES};

#[cfg(test)]
#[path = "reject_tests.rs"]
mod tests;

/// Why [`attachment_from_data_url`](super::attachment_from_data_url) failed.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::image_clipboard::{RejectReason, reject_reason};
///
/// let huge = format!("data:image/png;base64,{}", "A".repeat(20_000_000));
/// assert_eq!(reject_reason(&huge), Some(RejectReason::TooLarge));
/// assert_eq!(reject_reason("hello"), None);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RejectReason {
    /// Payload exceeds the decoded-size cap.
    TooLarge,
    /// The MIME type is not a supported image format.
    UnsupportedMime,
    /// The base64 payload is absent or malformed.
    MalformedBase64,
}

impl RejectReason {
    /// A user-facing explanation with the concrete limit.
    pub fn message(self) -> String {
        let mb = MAX_IMAGE_DECODED_BYTES / (1024 * 1024);
        match self {
            Self::TooLarge => {
                format!(
                    "Image too large to attach (limit {mb} MB). Resize it or use /image <path>."
                )
            }
            Self::UnsupportedMime => {
                "Unsupported image type. Use png, jpeg, gif, webp, bmp, or svg.".to_string()
            }
            Self::MalformedBase64 => {
                "Pasted image data was incomplete or corrupted; paste it again.".to_string()
            }
        }
    }
}

/// Classify a rejected image data URL, or `None` when it is not one.
pub fn reject_reason(text: &str) -> Option<RejectReason> {
    let data = text.trim().strip_prefix("data:")?;
    let (mime, payload) = data.split_once(";base64,")?;
    if !super::data_url::is_image_mime(mime) {
        return Some(RejectReason::UnsupportedMime);
    }
    if payload.len() > MAX_BASE64_PAYLOAD_CHARS {
        return Some(RejectReason::TooLarge);
    }
    super::attachment_from_data_url(text)
        .is_none()
        .then_some(RejectReason::MalformedBase64)
}
