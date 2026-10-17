//! execute for the token count tool.
use super::*;

pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let paths: Vec<String> =
        serde_json::from_value(args["paths"].clone()).context("paths must be string[]")?;
    let mut lines = Vec::with_capacity(paths.len());
    for path in paths {
        lines.push(match tokio::fs::read(&path).await {
            Ok(bytes) => describe(&path, &String::from_utf8_lossy(&bytes)),
            Err(e) => format!("{path}: error {e}"),
        });
    }
    Ok(ToolResult::success(lines.join("\n")))
}
