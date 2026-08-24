//! Approval-bound local and remote image loading.

use super::{ImageToolInput, local, remote};
use crate::tool::ToolResult;
use anyhow::Result;
use serde_json::{Value, json};

pub(super) async fn run(args: Value) -> Result<ToolResult> {
    let input: ImageToolInput = serde_json::from_value(args.clone())?;
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("image", &args).await {
        return Ok(blocked);
    }
    if let Err(error) = crate::approval::use_once::claim("image", &args) {
        return Ok(ToolResult::error(format!("approval claim failed: {error}")));
    }
    let (data_url, mime_type, size_bytes, source) = if remote_path(&input.path) {
        remote::load(&input.path, &args).await?
    } else {
        local::load(&input.path, &args).await?
    };
    let detail = input.detail.unwrap_or_else(|| "auto".to_string());
    let summary = format!("Image loaded: {source} ({size_bytes} bytes, {mime_type}, detail={detail})");
    Ok(ToolResult::success(summary).with_metadata(
        "image_data_url",
        json!({
            "data_url": data_url,
            "mime_type": mime_type,
            "detail": detail,
        }),
    ))
}

fn remote_path(path: &str) -> bool {
    path.starts_with("http://") || path.starts_with("https://")
}