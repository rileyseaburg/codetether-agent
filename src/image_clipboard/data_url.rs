//! Parse pasted image data URLs.

use base64::Engine;
use std::borrow::Cow;

use crate::session::ImageAttachment;

const MAX_IMAGE_DECODED_BYTES: usize = 10 * 1024 * 1024;
pub(crate) const MAX_BASE64_PAYLOAD_CHARS: usize = (MAX_IMAGE_DECODED_BYTES + 2) / 3 * 4;

const IMAGE_MIME_TYPES: &[&str] = &[
    "image/png",
    "image/jpeg",
    "image/gif",
    "image/webp",
    "image/bmp",
    "image/svg+xml",
];

/// Convert pasted image data URL text into an attachment.
///
/// # Arguments
///
/// * `text` - Candidate pasted data URL text.
///
/// # Returns
///
/// An [`ImageAttachment`] when `text` is a valid base64 image data URL.
///
/// # Examples
///
/// ```rust
/// use base64::Engine;
/// use codetether_agent::image_clipboard::attachment_from_data_url;
///
/// let payload = base64::engine::general_purpose::STANDARD.encode("png bytes");
/// let image = attachment_from_data_url(&format!("data:image/png;base64,{payload}"));
/// assert_eq!(image.unwrap().mime_type.as_deref(), Some("image/png"));
/// ```
pub fn attachment_from_data_url(text: &str) -> Option<ImageAttachment> {
    let data = text.trim().strip_prefix("data:")?;
    let (mime, raw_payload) = data.split_once(";base64,")?;
    if !IMAGE_MIME_TYPES.contains(&mime) {
        return None;
    }
    let payload = normalized_payload(raw_payload)?;
    if payload.len() > MAX_BASE64_PAYLOAD_CHARS {
        return None;
    }
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(payload.as_ref())
        .ok()?;
    if decoded.len() > MAX_IMAGE_DECODED_BYTES {
        return None;
    }
    Some(ImageAttachment {
        data_url: format!("data:{mime};base64,{payload}"),
        mime_type: Some(mime.to_string()),
    })
}

fn normalized_payload(payload: &str) -> Option<Cow<'_, str>> {
    let payload = payload.trim();
    if payload.is_empty() {
        return None;
    }
    if !payload.as_bytes().iter().any(u8::is_ascii_whitespace) {
        return Some(Cow::Borrowed(payload));
    }
    let payload: String = payload.split_ascii_whitespace().collect();
    (!payload.is_empty()).then_some(Cow::Owned(payload))
}
