//! Preserve tool-owned image attachments separately from rendered text.
//!
//! Producers use [`encoded`] for captured bytes. Runtime recorders use
//! [`content`] before text compaction so images remain in their tool message.

use crate::provider::ContentPart;
use serde_json::Value;
use std::collections::HashMap;

#[path = "result_images/encode.rs"]
mod encode;
pub(crate) use encode::encoded;

/// Read the canonical single-image or multi-image metadata without parsing prose.
pub(crate) fn content(metadata: Option<&HashMap<String, Value>>) -> Vec<ContentPart> {
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
#[path = "result_images/tests.rs"]
mod tests;
