//! info for filesystem inspection.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let path = match args["path"].as_str() {
        Some(p) => p,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "fileinfo",
                "path is required",
                Some(vec!["path"]),
                Some(json!({"path": "src/main.rs"})),
            ));
        }
    };

    let path_obj = Path::new(path);
    let metadata = fs::metadata(path).await?;

    let mut info = info_metadata::describe(path, &metadata);
    info_content::append(path_obj, path, &metadata, &mut info).await;

    Ok(ToolResult::success(info.join("\n")))
}
