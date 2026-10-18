//! Lossy UTF-8 decoding, bounded content and fenced text formatting.
use crate::size::format_bytes;
use std::path::Path;
pub(crate) fn format_text(
    path: &Path,
    display_path: &str,
    max_bytes: usize,
    bytes: &[u8],
) -> (String, bool, bool) {
    let mut text = String::from_utf8_lossy(bytes).to_string();
    let mut truncated = false;
    if text.len() > max_bytes {
        let mut end = max_bytes;
        while end > 0 && !text.is_char_boundary(end) {
            end -= 1;
        }
        text.truncate(end);
        truncated = true;
    }

    let language = path
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty())
        .unwrap_or("text");

    let mut snippet = format!("Shared file: {display_path}\n~~~{language}\n{text}\n~~~");
    if truncated {
        snippet.push_str(&format!(
            "\n[truncated to first {} bytes; original size: {}]",
            max_bytes,
            format_bytes(bytes.len() as u64)
        ));
    }

    (snippet, truncated, false)
}
