//! Encode captured image bytes into canonical tool-owned metadata.

use base64::Engine;
use serde_json::{Value, json};

/// Encode image bytes without embedding them into the tool's text output.
/// Encode bytes as an image metadata object.
///
/// # Arguments
/// * `bytes` - Raw image bytes.
/// * `mime` - MIME label placed in the data URL.
///
/// # Returns
/// The normalized metadata, provider parts, or rendered text described above.
///
/// # Examples
/// ```
/// let value = codetether_tool_core::images::encoded(&[1,2,3], "image/png");
/// assert_eq!(value["data_url"], "data:image/png;base64,AQID");
/// ```
pub fn encoded(bytes: &[u8], mime: &str) -> Value {
    let data = base64::engine::general_purpose::STANDARD.encode(bytes);
    json!({
        "data_url": format!("data:{mime};base64,{data}"),
        "mime_type": mime,
    })
}
