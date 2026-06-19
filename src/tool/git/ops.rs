//! Maps the `op` argument to a concrete read-only `git` invocation.

use super::run::{cwd_of, run_git};
use crate::tool::ToolResult;
use serde_json::Value;

/// Execute a read-only git operation (`status`, `diff`, `log`, `branch`,
/// `show`). Mutating operations are handled in [`super::commit`].
pub(super) async fn run_readonly(op: &str, args: &Value) -> anyhow::Result<ToolResult> {
    let cwd = cwd_of(args);
    let path = args.get("path").and_then(|v| v.as_str());
    let argv: Vec<&str> = match op {
        "status" => vec!["status", "--short", "--branch"],
        "diff" => match path {
            Some(p) => vec!["diff", "--", p],
            None => vec!["diff"],
        },
        "diff_staged" => vec!["diff", "--staged"],
        "log" => vec!["log", "--oneline", "-n", "20"],
        "branch" => vec!["branch", "--show-current"],
        "show" => vec!["show", "--stat", "--oneline", "HEAD"],
        other => {
            return Ok(ToolResult::error(format!(
                "Unknown git op '{other}'. Use status|diff|diff_staged|log|branch|show|commit."
            )));
        }
    };
    let (out, ok) = run_git(&cwd, &argv).await?;
    let body = if out.trim().is_empty() {
        "(no output)".to_string()
    } else {
        out
    };
    Ok(if ok {
        ToolResult::success(body).truncate_to(16_000)
    } else {
        ToolResult::error(body).truncate_to(16_000)
    })
}
