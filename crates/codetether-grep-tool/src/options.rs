//! options for bounded grep searches.
use super::*;
pub(super) fn parse(args: &Value) -> Result<Options<'_>, ToolResult> {
    let pattern = match args["pattern"].as_str() {
        Some(p) => p,
        None => {
            return Err(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "grep",
                "pattern is required",
                Some(vec!["pattern"]),
                Some(json!({"pattern": "search text", "path": "src/"})),
            ));
        }
    };
    let search_path = args["path"].as_str().unwrap_or(".");
    let is_regex = args["is_regex"].as_bool().unwrap_or(false);
    let include = args["include"].as_str();
    let limit = args["limit"]
        .as_u64()
        .map(|n| n as usize)
        .unwrap_or(DEFAULT_GREP_LIMIT)
        .clamp(1, MAX_GREP_LIMIT);
    let timeout_secs = args["timeout_secs"]
        .as_u64()
        .or_else(|| env_u64("CODETETHER_GREP_TIMEOUT_SECS"))
        .unwrap_or(DEFAULT_GREP_TIMEOUT_SECS)
        .clamp(1, MAX_GREP_TIMEOUT_SECS);
    let max_scanned_files = env_usize("CODETETHER_GREP_MAX_SCANNED_FILES")
        .unwrap_or(DEFAULT_GREP_MAX_SCANNED_FILES)
        .max(1);
    let max_file_bytes = env_u64("CODETETHER_GREP_MAX_FILE_BYTES")
        .unwrap_or(DEFAULT_GREP_MAX_FILE_BYTES)
        .max(1);

    Ok(Options {
        pattern,
        search_path,
        is_regex,
        include,
        limit,
        timeout_secs,
        max_scanned_files,
        max_file_bytes,
    })
}
