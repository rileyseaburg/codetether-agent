//! Promote nested batch image metadata to the batch call's own attachments.

use crate::tool::ToolResult;
use serde_json::{Value, json};

pub(super) fn attach(mut result: ToolResult, calls: Vec<Value>) -> ToolResult {
    let mut images = Vec::new();
    for call in &calls {
        if let Some(image) = call.get("metadata").and_then(|m| m.get("image_data_url")) {
            match image {
                Value::Array(parts) => images.extend(parts.iter().cloned()),
                image => images.push(image.clone()),
            }
        }
    }
    if !calls.is_empty() {
        result = result.with_metadata("calls", json!(calls));
    }
    if images.is_empty() {
        result
    } else {
        result.with_metadata("image_data_url", json!(images))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn batch_images_include_failed_nested_calls_in_order() {
        let first = ToolResult::success("first").with_metadata(
            "image_data_url",
            json!({"data_url":"data:image/png;base64,AQID"}),
        );
        let second = ToolResult::error("second").with_metadata(
            "image_data_url",
            json!([{"data_url":"data:image/png;base64,BAUG"}]),
        );
        let result = super::super::build(vec![
            (0, "image".into(), first),
            (1, "image".into(), second),
        ]);
        assert!(!result.success);
        assert_eq!(
            result.metadata["image_data_url"].as_array().unwrap().len(),
            2
        );
        assert!(!result.output.contains("base64"));
    }
}
