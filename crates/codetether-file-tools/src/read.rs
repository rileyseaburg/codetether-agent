//! read for core file tools.
use super::*;
pub(super) async fn execute(args: Value) -> Result<ToolResult> {
    let start = Instant::now();

    let path = match args["path"].as_str() {
        Some(p) => p,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "read",
                "path is required",
                Some(vec!["path"]),
                Some(json!({"path": "src/main.rs"})),
            ));
        }
    };
    let offset = args["offset"].as_u64().map(|n| n as usize);
    let limit = args["limit"].as_u64().map(|n| n as usize);

    let content = fs::read_to_string(path).await?;

    let lines: Vec<&str> = content.lines().collect();
    let start_line = offset
        .map(|o| o.saturating_sub(1))
        .unwrap_or(0)
        .min(lines.len());
    let end_line = limit
        .map(|l| (start_line + l).min(lines.len()))
        .unwrap_or(lines.len());

    let mut selected: String = lines[start_line..end_line]
        .iter()
        .enumerate()
        .map(|(i, line)| format!("{:4} | {}", start_line + i + 1, line))
        .collect::<Vec<_>>()
        .join("\n");

    read_telemetry::record(path, offset, limit, start_line, end_line, start);
    if limit.is_none() {
        selected = selected.chars().take(262_144).collect();
    }
    Ok(ToolResult::success(selected)
        .with_metadata("total_lines", json!(lines.len()))
        .with_metadata("read_lines", json!(end_line - start_line)))
}
