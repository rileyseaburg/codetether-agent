//! Full invocation detail for interactive approval review.

use serde_json::Value;

/// Renders the exact command, patch, or serialized arguments without clipping.
pub(super) fn render(tool: &str, args: &Value) -> Option<String> {
    match tool {
        "apply_patch" | "patch" => field(args, "patch"),
        "bash" => field(args, "command"),
        "exec_command" => field(args, "cmd"),
        "write" => write_detail(args),
        _ => serde_json::to_string_pretty(args).ok(),
    }
    .filter(|detail| !detail.trim().is_empty())
}

fn write_detail(args: &Value) -> Option<String> {
    let path = field(args, "path")?;
    let content = field(args, "content").unwrap_or_default();
    Some(format!("path: {path}\n\n{content}"))
}

fn field(args: &Value, name: &str) -> Option<String> {
    args.get(name)?.as_str().map(str::to_string)
}

#[cfg(test)]
#[path = "invocation_detail_tests.rs"]
mod tests;
