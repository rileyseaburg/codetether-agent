//! Encode captured image bytes into canonical tool-owned metadata.

use base64::Engine;
use serde_json::{Value, json};

/// Encode image bytes without embedding them into the tool's text output.
pub(crate) fn encoded(bytes: &[u8], mime: &str) -> Value {
    let data = base64::engine::general_purpose::STANDARD.encode(bytes);
    json!({
        "data_url": format!("data:{mime};base64,{data}"),
        "mime_type": mime,
    })
}
