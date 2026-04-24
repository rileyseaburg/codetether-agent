//! File preview: reads the first N bytes/lines of a file for picker display.

use std::io::Read;
use std::path::Path;

use crate::tui::utils::helpers::truncate_with_ellipsis;

/// Read up to `max_bytes` bytes, returning up to `max_lines` lines.
///
/// Returns `(lines, was_truncated, is_binary)`.
pub fn read_file_preview_lines(
    path: &Path,
    max_bytes: usize,
    max_lines: usize,
) -> Result<(Vec<String>, bool, bool), std::io::Error> {
    let mut file = std::fs::File::open(path)?;
    let mut buffer = vec![0_u8; max_bytes.saturating_add(1)];
    let bytes_read = file.read(&mut buffer)?;

    let truncated_by_bytes = bytes_read > max_bytes;
    buffer.truncate(bytes_read.min(max_bytes));

    if buffer.contains(&0) {
        return Ok((Vec::new(), truncated_by_bytes, true));
    }

    let text = String::from_utf8_lossy(&buffer).to_string();
    let mut lines: Vec<String> = text
        .lines()
        .map(|line| truncate_with_ellipsis(line, 220))
        .collect();

    if lines.is_empty() {
        lines.push("(empty file)".to_string());
    }

    let mut truncated = truncated_by_bytes;
    if lines.len() > max_lines {
        lines.truncate(max_lines);
        truncated = true;
    }

    Ok((lines, truncated, false))
}
