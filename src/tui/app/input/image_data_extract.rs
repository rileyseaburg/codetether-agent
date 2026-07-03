//! Extract an embedded `data:image/...;base64,` URL from arbitrary text.

/// A data URL found inside a larger text buffer.
pub(crate) struct ExtractedImage {
    /// The full `data:image/...;base64,...` URL.
    pub data_url: String,
    /// The input text with the data URL removed and trimmed.
    pub remainder: String,
}

const MARKER: &str = ";base64,";

/// Find the first embedded image data URL and split it from `text`.
///
/// Over SSH a pasted image arrives as a long data-URL string that may be
/// mixed with caption text or arrive without a clean bracketed-paste event.
pub(crate) fn extract_image_data_url(text: &str) -> Option<ExtractedImage> {
    let start = text.find("data:image/")?;
    let payload_start = start + text[start..].find(MARKER)? + MARKER.len();
    let payload_len = base64_run_len(&text[payload_start..]);
    if payload_len == 0 {
        return None;
    }
    let end = payload_start + payload_len;
    let mut remainder = String::with_capacity(text.len());
    remainder.push_str(&text[..start]);
    remainder.push_str(&text[end..]);
    Some(ExtractedImage {
        data_url: text[start..end].to_string(),
        remainder: remainder.trim().to_string(),
    })
}

/// Length of the leading base64 run (payload may contain wrapped whitespace).
fn base64_run_len(s: &str) -> usize {
    s.bytes()
        .take_while(|b| {
            b.is_ascii_alphanumeric() || matches!(b, b'+' | b'/' | b'=' | b'\n' | b'\r')
        })
        .count()
}
