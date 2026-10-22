//! Decode image data URLs with payload and MIME checks.
use crate::ImageAttachment;
use base64::Engine;
mod limits;
mod normalize;
use limits::IMAGE_MIME_TYPES;
pub(crate) use limits::{MAX_BASE64_PAYLOAD_CHARS, MAX_IMAGE_DECODED_BYTES};
use normalize::normalized_payload;

/// True when `mime` is a supported image media type.
pub(crate) fn is_image_mime(mime: &str) -> bool {
    IMAGE_MIME_TYPES.contains(&mime)
}

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
/// use codetether_image::attachment_from_data_url;
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
