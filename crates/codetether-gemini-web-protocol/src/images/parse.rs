use super::{ImageResponse, attachments, framing, path::at, status};
use anyhow::{Result, anyhow, ensure};
use serde_json::Value;

/// Decode a complete Gemini image response without performing network I/O.
///
/// # Arguments
/// * `body` - Decompressed response bytes, including the anti-XSSI prefix and
///   byte-length-prefixed `wrb.fr` batches (not SSE).
///
/// # Returns
/// Raw image URLs from the latest candidate snapshot and unambiguous state for
/// refinement. Unknown frames and opaque `e` trailers do not imply errors.
///
/// # Errors
/// Returns fixed messages for malformed/truncated framing or JSON, explicit
/// `er` frames, error answers, absent images, or more than eight distinct images.
/// Errors never include response content, signed URLs, or supplied credentials.
///
/// # Examples
/// ```
/// use codetether_gemini_web_protocol::images::parse_response;
/// assert!(parse_response(b"not a framed response").is_err());
/// ```
pub fn parse_response(body: &[u8]) -> Result<ImageResponse> {
    let frames = framing::decode(body)?;
    let mut response = ImageResponse {
        urls: Vec::new(),
        continuation: None,
    };
    let mut answer = String::new();
    let mut rpc_error = false;
    for frame in frames {
        let kind = frame.get(0).and_then(Value::as_str);
        rpc_error |= kind == Some("er");
        if kind != Some("wrb.fr") || frame.get(2) == Some(&Value::Null) {
            continue;
        }
        let encoded = frame
            .get(2)
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("generation_payload_error"))?;
        let inner: Value =
            serde_json::from_str(encoded).map_err(|_| anyhow!("generation_payload_error"))?;
        if let Some(text) = at(&inner, &[4, 0, 1, 0])
            .and_then(Value::as_str)
            .filter(|text| !text.trim().is_empty())
        {
            answer = text.to_owned();
        }
        if let Some(snapshot) = attachments::snapshot(&inner) {
            response = snapshot;
        }
    }
    ensure!(!rpc_error, "generation_rpc_error");
    status::check(&answer, response.urls.len())?;
    Ok(response)
}
