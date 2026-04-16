//! Response-assembly helpers: convert HTTP results into `ToolResult`.

use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::path::Path;

pub(super) fn response_result(
    action: &str,
    base_url: &str,
    path: &str,
    status: u16,
    body: Value,
) -> Result<ToolResult> {
    let pretty = serde_json::to_string_pretty(&body).unwrap_or_else(|_| body.to_string());
    let success = (200..300).contains(&status);
    let mut metadata = HashMap::new();
    metadata.insert("action".to_string(), json!(action));
    metadata.insert("base_url".to_string(), json!(base_url));
    metadata.insert("endpoint".to_string(), json!(path));
    metadata.insert("http_status".to_string(), json!(status));
    for key in ["url", "title", "path", "active_index"] {
        if let Some(v) = body.get(key) {
            metadata.insert(key.to_string(), v.clone());
        }
    }
    Ok(ToolResult {
        output: pretty,
        success,
        metadata,
    })
}

pub(super) fn screenshot_metadata(body: &Value) -> Option<Value> {
    let path = body.get("path")?.as_str()?;
    let path_obj = Path::new(path);
    Some(json!({
        "path": path,
        "exists": path_obj.exists(),
        "absolute": path_obj.is_absolute(),
    }))
}
