//! Preserve tool-owned image attachments separately from rendered text.
//!
//! Producers use [`encoded`] for captured bytes. Runtime recorders use
//! [`content`] before text compaction so images remain in their tool message.

use codetether_provider_core::ContentPart;
use serde_json::Value;
use std::collections::HashMap;

mod encode;
pub use encode::encoded;

/// Read the canonical single-image or multi-image metadata without parsing prose.
/// Extract provider image parts from canonical tool metadata.
///
/// # Arguments
/// * `metadata` - Optional metadata containing a single image or an image array.
///
/// # Returns
/// The normalized metadata, provider parts, or rendered text described above.
///
/// # Examples
/// ```
/// assert!(codetether_tool_core::images::content(None).is_empty());
/// ```
pub fn content(metadata: Option<&HashMap<String, Value>>) -> Vec<ContentPart> {
    let Some(value) = metadata.and_then(|map| map.get("image_data_url")) else {
        return Vec::new();
    };
    match value {
        Value::Array(images) => images.iter().filter_map(image).collect(),
        value => image(value).into_iter().collect(),
    }
}

fn image(value: &Value) -> Option<ContentPart> {
    let url = value.as_str().or_else(|| value.get("data_url")?.as_str())?;
    if url.trim().is_empty() {
        return None;
    }
    let mime_type = value
        .get("mime_type")
        .and_then(Value::as_str)
        .map(str::to_owned);
    Some(ContentPart::Image {
        url: url.to_owned(),
        mime_type,
    })
}

#[cfg(test)]
mod tests;
