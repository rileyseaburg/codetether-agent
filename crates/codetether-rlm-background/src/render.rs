//! Rendering helpers for foreground and cached RLM summaries.

/// Render a completed RLM summary.
pub(super) fn summary(tool: &str, original_bytes: usize, reason: &str, body: &str) -> String {
    format!(
        "[RLM-SUMMARY tool={tool} original_bytes={original_bytes} reason={reason}]\n{body}\n[END RLM-SUMMARY - raw output was replaced by a cached background summary]"
    )
}

/// Render immediate fallback while the background job runs.
pub(super) fn pending(
    key: u64,
    tool: &str,
    original_bytes: usize,
    reason: &str,
    truncated: String,
) -> String {
    format!(
        "[RLM-BACKGROUND key={key:016x} tool={tool} original_bytes={original_bytes} reason={reason}]\nRLM summarisation is running in the background; using bounded output for this turn.\n{truncated}\n[END RLM-BACKGROUND]"
    )
}
