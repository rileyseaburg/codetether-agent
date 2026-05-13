//! Source label extraction for structured payloads.

use crate::router::CrateAutoProcessContext;

/// Return a stable file/source label for oracle payloads.
pub fn source_label(ctx: &CrateAutoProcessContext<'_>) -> String {
    for key in ["filePath", "path", "file"] {
        if let Some(path) = ctx.tool_args.get(key).and_then(|v| v.as_str()) {
            return path.to_string();
        }
    }
    first_path(ctx).unwrap_or_else(|| "inline".into())
}

fn first_path(ctx: &CrateAutoProcessContext<'_>) -> Option<String> {
    ctx.tool_args
        .get("paths")
        .and_then(|v| v.as_array())
        .and_then(|arr| arr.iter().find_map(|v| v.as_str()))
        .map(str::to_string)
}
