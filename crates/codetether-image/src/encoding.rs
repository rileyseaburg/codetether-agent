//! Base64 data-URL construction and payload size inspection.
//! These helpers retain caller-supplied MIME labels without validating image bytes.
use base64::Engine;

/// Encode bytes using standard base64 and the supplied MIME label.
///
/// # Arguments
/// * `mime_type` — Label to place in the data URL.
/// * `bytes` — Payload to encode.
///
/// # Returns
/// A data URL containing the encoded payload.
///
/// # Examples
/// ```
/// assert_eq!(codetether_image::encode_data_url("image/png", b"x"), "data:image/png;base64,eA==");
/// ```
pub fn encode(mime_type: &str, bytes: &[u8]) -> String {
    let payload = base64::engine::general_purpose::STANDARD.encode(bytes);
    format!("data:{mime_type};base64,{payload}")
}

/// Decode a base64 URL payload and report its byte length.
///
/// # Arguments
/// * `data_url` — Text containing a `;base64,` separator.
///
/// # Returns
/// Decoded byte length, or None for a missing separator or invalid base64.
///
/// # Examples
/// ```
/// assert_eq!(codetether_image::decoded_len("data:image/png;base64,eA=="), Some(1));
/// ```
pub fn decoded_len(data_url: &str) -> Option<usize> {
    let (_, payload) = data_url.split_once(";base64,")?;
    base64::engine::general_purpose::STANDARD
        .decode(payload.trim())
        .ok()
        .map(|bytes| bytes.len())
}
