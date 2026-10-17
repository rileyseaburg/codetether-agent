//! write for core file tools.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let start = Instant::now();

    let args = match file_write_args::WriteArgs::parse(&args) {
        Ok(parsed) => parsed,
        Err(error) => return Ok(error),
    };
    let (path, content) = (args.path, args.content);
    // Create parent directories if needed
    if let Some(parent) = PathBuf::from(path).parent() {
        fs::create_dir_all(parent).await?;
    }

    // Check if file exists for telemetry
    let existed = fs::metadata(path).await.is_ok();
    let old_content = if existed {
        fs::read_to_string(path).await.ok()
    } else {
        None
    };

    fs::write(path, content).await?;

    let duration = start.elapsed();

    write_telemetry::record(path, content, existed, &old_content, duration);
    Ok(ToolResult::success(format!(
        "Wrote {} bytes to {}",
        content.len(),
        path
    )))
}
