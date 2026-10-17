//! Render matching file lines using one-based line numbers.
use anyhow::Result;
use std::path::Path;

pub(super) fn file_matches(path: &Path, matcher: &regex::Regex) -> Result<Vec<String>> {
    let content = std::fs::read_to_string(path)?;
    Ok(content
        .lines()
        .enumerate()
        .filter(|(_, line)| matcher.is_match(line))
        .map(|(idx, line)| format!("{}:{}:{}", path.display(), idx + 1, line))
        .collect())
}
