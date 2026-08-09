//! Porcelain status line parsing for worktree artifact inventory.

/// Extracts the path from one `git status --porcelain` line.
///
/// Skips ignored entries and resolves rename records (`R old -> new`) to
/// their destination path.
pub(super) fn changed_path(line: &str) -> Option<String> {
    if line.len() < 4 || line.starts_with("!!") {
        return None;
    }
    let path = line[3..].trim();
    let path = path.rsplit(" -> ").next().unwrap_or(path);
    let path = path.trim_matches('"');
    (!path.is_empty()).then(|| path.to_string())
}
