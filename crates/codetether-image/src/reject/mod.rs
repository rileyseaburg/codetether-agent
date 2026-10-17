//! Classify rejected data URLs and provide display explanations.
mod reason;
mod message;
pub use reason::RejectReason;
use super::data_url::MAX_BASE64_PAYLOAD_CHARS;
/// Classify a rejected image data URL, or `None` when it is not one.
///
/// # Arguments
/// * `text` - Candidate pasted data URL.
///
/// # Returns
/// A rejection reason, or `None` for accepted input or text outside this format.
///
/// # Examples
/// ```
/// use codetether_image::{RejectReason, reject_reason};
/// assert_eq!(reject_reason("data:text/plain;base64,aA=="), Some(RejectReason::UnsupportedMime));
/// assert_eq!(reject_reason("ordinary text"), None);
/// ```
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
#[cfg(test)]
mod tests;
