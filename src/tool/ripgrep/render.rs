//! Turn raw ripgrep output into a [`ToolResult`].

use super::exec::Output;
use crate::tool::ToolResult;
use serde_json::json;

/// Format `output`, truncating to `limit` lines.
///
/// Ripgrep exit codes: `0` = matches, `1` = no matches (not an error), other =
/// real failure. A `None` code means the wall-clock budget was exhausted.
pub(super) fn render(output: Output, limit: usize) -> ToolResult {
    match output.code {
        None => ToolResult::error("`rg` timed out; narrow the pattern or add --glob filters"),
        Some(0) => matches(output.stdout, limit),
        Some(1) => ToolResult::success("No matches found").with_metadata("matches", json!(0)),
        Some(code) => ToolResult::error(format!(
            "rg exited with code {code}: {}",
            output.stderr.trim()
        )),
    }
}

fn matches(stdout: String, limit: usize) -> ToolResult {
    let all: Vec<&str> = stdout.lines().collect();
    let total = all.len();
    let shown = total.min(limit);
    let mut body = all[..shown].join("\n");
    if total > shown {
        body.push_str(&format!(
            "\n… {} more line(s) truncated; raise `limit` or narrow the pattern",
            total - shown
        ));
    }
    ToolResult::success(body)
        .with_metadata("matches", json!(total))
        .with_metadata("truncated", json!(total > shown))
}
