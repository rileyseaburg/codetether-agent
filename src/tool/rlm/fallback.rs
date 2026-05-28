//! Deterministic fallback responses for failed RLM processing.

use super::ToolResult;
use crate::rlm::{RlmChunker, RlmRouter};
use serde_json::json;

pub(super) fn non_converged(action: &str, content: &str) -> ToolResult {
    let truncated = truncate(action, content);
    ToolResult::success(format!(
        "RLM {action} (degraded mode - auto_process did not converge)\n\n{truncated}"
    ))
}

pub(super) fn failed(action: &str, content: &str, error: anyhow::Error) -> ToolResult {
    let truncated = truncate(action, content);
    ToolResult::success(format!(
        "RLM {action} (fallback mode - auto_process failed: {error})\n\n{truncated}"
    ))
}

fn truncate(action: &str, content: &str) -> String {
    let input_tokens = RlmChunker::estimate_tokens(content);
    let (truncated, _, _) = RlmRouter::smart_truncate(
        content,
        action,
        &json!({}),
        input_tokens.min(8000),
    );
    truncated
}
