//! render for bounded grep searches.
use super::*;
pub(super) fn render(scanned: Scanned, options: &Options<'_>) -> Result<ToolResult> {
    let Scanned {
        results,
        scanned_files,
        skipped_oversize,
        skipped_unreadable,
        timed_out,
        scan_limit_reached,
    } = scanned;
    let (limit, timeout_secs, max_scanned_files) = (
        options.limit,
        options.timeout_secs,
        options.max_scanned_files,
    );
    let result_limit_reached = results.len() >= limit;
    let truncated = result_limit_reached || timed_out || scan_limit_reached;
    let mut output = results.join("\n");
    if output.is_empty() {
        output = "No matches found".to_string();
    }
    if timed_out {
        output.push_str(&format!(
                "\n[grep stopped after {timeout_secs}s; scanned {scanned_files} files. Narrow path/include or raise timeout_secs.]"
            ));
    } else if scan_limit_reached {
        output.push_str(&format!(
            "\n[grep stopped after scanning {max_scanned_files} files. Narrow path/include.]"
        ));
    }

    let result = if timed_out || scan_limit_reached {
        ToolResult::error(output)
    } else {
        ToolResult::success(output)
    };

    Ok(result
        .with_metadata("count", json!(results.len()))
        .with_metadata("truncated", json!(truncated))
        .with_metadata("scanned_files", json!(scanned_files))
        .with_metadata("skipped_oversize", json!(skipped_oversize))
        .with_metadata("skipped_unreadable", json!(skipped_unreadable))
        .with_metadata("timed_out", json!(timed_out))
        .with_metadata("scan_limit_reached", json!(scan_limit_reached)))
}
