//! `commit` operation: optionally stage paths, then create a commit.

use super::run::{cwd_of, run_git};
use crate::tool::ToolResult;
use serde_json::Value;

/// Stage files (all tracked changes, or specific `paths`) and commit with
/// `message`. Returns an error result if `message` is missing or git fails.
pub(super) async fn run_commit(args: &Value) -> anyhow::Result<ToolResult> {
    let cwd = cwd_of(args);
    let Some(message) = args.get("message").and_then(|v| v.as_str()) else {
        return Ok(ToolResult::error("commit requires a 'message' argument"));
    };
    let paths: Vec<String> = args
        .get("paths")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|p| p.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    let mut add: Vec<&str> = vec!["add"];
    if paths.is_empty() {
        add.push("-A");
    } else {
        add.extend(paths.iter().map(String::as_str));
    }
    let (add_out, add_ok) = run_git(&cwd, &add).await?;
    if !add_ok {
        return Ok(ToolResult::error(format!("git add failed: {add_out}")));
    }
    let (out, ok) = run_git(&cwd, &["commit", "-m", message]).await?;
    Ok(if ok {
        ToolResult::success(out).truncate_to(8_000)
    } else {
        ToolResult::error(out).truncate_to(8_000)
    })
}
