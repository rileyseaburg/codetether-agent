//! preview for filesystem inspection.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let path = match args["path"].as_str() {
        Some(p) => p,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "headtail",
                "path is required",
                Some(vec!["path"]),
                Some(json!({"path": "src/main.rs", "head": 10})),
            ));
        }
    };
    let head_lines = args["head"].as_u64().unwrap_or(10) as usize;
    let tail_lines = args["tail"].as_u64().unwrap_or(0) as usize;

    let content = fs::read_to_string(path).await?;
    Ok(preview_view::render(path, &content, head_lines, tail_lines))
}
