//! list for core file tools.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let path = match args["path"].as_str() {
        Some(p) => p,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "list",
                "path is required",
                Some(vec!["path"]),
                Some(json!({"path": "src/"})),
            ));
        }
    };

    let mut entries = fs::read_dir(path).await?;
    let mut items = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        let name = entry.file_name().to_string_lossy().to_string();
        let file_type = entry.file_type().await?;

        let suffix = if file_type.is_dir() {
            "/"
        } else if file_type.is_symlink() {
            "@"
        } else {
            ""
        };

        items.push(format!("{}{}", name, suffix));
    }

    items.sort();
    Ok(ToolResult::success(items.join("\n")).with_metadata("count", json!(items.len())))
}
