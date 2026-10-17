//! tree for filesystem inspection.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let path = match args["path"].as_str() {
        Some(p) => p,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "tree",
                "path is required",
                Some(vec!["path"]),
                Some(json!({"path": "src/"})),
            ));
        }
    };
    let max_depth = args["depth"].as_u64().unwrap_or(3) as usize;
    let show_hidden = args["show_hidden"].as_bool().unwrap_or(false);
    let show_size = args["show_size"].as_bool().unwrap_or(false);
    let respect_gitignore = args["gitignore"].as_bool().unwrap_or(true);

    let mut output = Vec::new();
    let root_path = Path::new(path);

    // Add root directory
    output.push(format!(
        "{}/",
        root_path.file_name().unwrap_or_default().to_string_lossy()
    ));

    let mut file_count = 0;
    let mut dir_count = 0;

    // Build tree recursively
    build_tree(
        root_path,
        "",
        0,
        max_depth,
        show_hidden,
        show_size,
        respect_gitignore,
        &mut output,
        &mut file_count,
        &mut dir_count,
    )
    .await?;

    output.push(String::new());
    output.push(format!("{} directories, {} files", dir_count, file_count));

    Ok(ToolResult::success(output.join("\n"))
        .with_metadata("directories", json!(dir_count))
        .with_metadata("files", json!(file_count)))
}
