//! Rejection categories.
/// Why [`attachment_from_data_url`](crate::attachment_from_data_url) failed.
///
/// # Examples
///
/// ```rust
/// use codetether_image::{RejectReason, reject_reason};
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
