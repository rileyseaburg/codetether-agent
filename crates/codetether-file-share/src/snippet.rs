//! File reading and binary detection for attachment snippets.
use crate::size::format_bytes;
use anyhow::Result;
use std::path::Path;
/// Read a file into a shareable text block or a binary metadata label.
/// # Arguments
/// * `path` - File to read.
/// * `display_path` - Label placed in the snippet.
/// * `max_bytes` - Maximum retained UTF-8 content bytes.
/// # Returns
/// The snippet, whether text was truncated, and whether the file is binary.
/// # Errors
/// Returns the file read error when the file cannot be read.
/// # Examples
/// ```no_run
/// let (snippet, _, _) = codetether_file_share::build_file_share_snippet(
///     std::path::Path::new("README.md"), "README.md", 1024).unwrap();
/// assert!(snippet.starts_with("Shared file: README.md"));
/// ```
pub fn build_file_share_snippet(
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

    Ok(crate::text::format_text(
        path,
        display_path,
        max_bytes,
        &bytes,
    ))
}
