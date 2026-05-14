//! Bounded content collection for RLM invocations.

use crate::tool::ToolResult;
use anyhow::Result;

const PATH_READ_BUDGET_BYTES: usize = 20 * 1024 * 1024;

pub(super) async fn collect(paths: &[&str], content: Option<&str>) -> Result<String, ToolResult> {
    if let Some(c) = content {
        return Ok(c.to_string());
    }
    if paths.is_empty() {
        return Err(ToolResult::error("Either 'paths' or 'content' is required"));
    }

    let mut collected = String::new();
    for path in paths {
        append_path(&mut collected, path).await;
    }
    Ok(collected)
}

async fn append_path(collected: &mut String, path: &str) {
    if would_exceed_budget(collected.len(), path).await {
        collected.push_str(&format!(
            "=== {path} (skipped: would exceed memory budget) ===\n\n"
        ));
        return;
    }

    match tokio::fs::read_to_string(path).await {
        Ok(c) => collected.push_str(&format!("=== {path} ===\n{c}\n\n")),
        Err(e) => collected.push_str(&format!("=== {path} (error: {e}) ===\n\n")),
    }
}

async fn would_exceed_budget(current_len: usize, path: &str) -> bool {
    match tokio::fs::metadata(path).await {
        Ok(meta) => current_len.saturating_add(meta.len() as usize) > PATH_READ_BUDGET_BYTES,
        Err(_) => false,
    }
}
