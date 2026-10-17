//! render for workspace code search.
use super::*;
pub(super) fn render(all_matches: &[Match], p: &Params) -> Result<ToolResult> {
    if all_matches.is_empty() {
        return Ok(ToolResult::success(format!(
            "No matches found for pattern: {}",
            p.pattern
        )));
    }

    let output = all_matches
        .iter()
        .take(MAX_RESULTS)
        .map(|m| format!("{}:{}\n{}", m.path, m.line, m.context))
        .collect::<Vec<_>>()
        .join("\n\n");

    let truncated = all_matches.len() > MAX_RESULTS;
    let msg = if truncated {
        format!(
            "Found {} matches (showing first {}):\n\n{}",
            all_matches.len(),
            MAX_RESULTS,
            output
        )
    } else {
        format!("Found {} matches:\n\n{}", all_matches.len(), output)
    };

    Ok(ToolResult::success(msg).with_metadata("match_count", json!(all_matches.len())))
}
