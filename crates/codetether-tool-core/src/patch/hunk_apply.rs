//! In-memory application of one parsed hunk.

use super::types::PatchHunk;
use anyhow::{Result, anyhow};

/// Apply a single hunk to the provided file content.
///
/// # Arguments
/// * `content` — Existing file text.
/// * `hunk` — Parsed replacement or insertion.
/// # Returns
/// Updated text joined with newline separators.
/// # Errors
/// Returns an error when the old lines or insertion position cannot be matched.
/// # Examples
/// ```
/// let hunks = codetether_tool_core::patch::parse_patch("+++ b/note.txt\n@@ -1 +1 @@\n-old\n+new");
/// assert_eq!(codetether_tool_core::patch::apply_hunk("old", &hunks[0]).unwrap(), "new");
/// ```
pub fn apply(content: &str, hunk: &PatchHunk) -> Result<String> {
    let lines: Vec<&str> = content.lines().collect();
    let match_start =
        find_match(&lines, hunk).ok_or_else(|| anyhow!("Could not find hunk location"))?;
    let mut result = Vec::new();
    result.extend(lines[..match_start].iter().map(|line| line.to_string()));
    result.extend(hunk.new_lines.clone());
    result.extend(
        lines[match_start + hunk.old_lines.len()..]
            .iter()
            .map(|line| line.to_string()),
    );
    Ok(result.join("\n"))
}

fn find_match(lines: &[&str], hunk: &PatchHunk) -> Option<usize> {
    if hunk.old_lines.is_empty() {
        return (hunk.start_line <= lines.len()).then_some(hunk.start_line);
    }
    let max_start = lines.len().saturating_sub(hunk.old_lines.len());
    (0..=max_start).find(|start| hunk_matches(lines, hunk, *start))
}

fn hunk_matches(lines: &[&str], hunk: &PatchHunk, start: usize) -> bool {
    hunk.old_lines
        .iter()
        .enumerate()
        .all(|(offset, old_line)| line_matches(lines, start + offset, old_line))
}

fn line_matches(lines: &[&str], index: usize, old_line: &str) -> bool {
    index < lines.len() && lines[index].trim() == old_line.trim()
}

#[cfg(test)]
#[path = "hunk_apply_tests.rs"]
mod tests;
