//! Standalone file picker helper functions
//!
//! Read preview, display path normalization, and file share snippet generation.

use std::io::{Read, Seek, SeekFrom};
use std::path::Path;


fn read_file_preview_lines(
    path: &Path,
    max_bytes: usize,
    max_lines: usize,
) -> Result<(Vec<String>, bool, bool)> {
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

fn display_path_for_workspace(path: &Path, workspace_dir: &Path) -> String {
    path.strip_prefix(workspace_dir)
        .unwrap_or(path)
        .display()
        .to_string()
}

/// Build a text snippet that can be inserted into the composer to share a file with the model.
/// Returns (snippet, truncated, binary).
fn build_file_share_snippet(
    path: &Path,
    display_path: &str,
    max_bytes: usize,
) -> Result<(String, bool, bool)> {
    let bytes = std::fs::read(path)?;

    if bytes.contains(&0) {
        let snippet = format!(
            "Shared file: {display_path}\n[binary file, size: {}]",
            format_bytes(bytes.len() as u64)
        );
        return Ok((snippet, false, true));
    }

    let mut text = String::from_utf8_lossy(&bytes).to_string();
    let mut truncated = false;
    if text.len() > max_bytes {
        let mut end = max_bytes;
        while end > 0 && !text.is_char_boundary(end) {
            end -= 1;
        }
        text.truncate(end);
        truncated = true;
    }

    let language = path
        .extension()
        .and_then(|ext| ext.to_str())
        .filter(|ext| !ext.is_empty())
        .unwrap_or("text");

    let mut snippet = format!("Shared file: {display_path}\n~~~{language}\n{text}\n~~~");
    if truncated {
        snippet.push_str(&format!(
            "\n[truncated to first {} bytes; original size: {}]",
            max_bytes,
            format_bytes(bytes.len() as u64)
        ));
    }

    Ok((snippet, truncated, false))
}

