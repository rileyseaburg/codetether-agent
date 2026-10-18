//! Concise status, unstaged diff and staged diff report.
use super::report_query::git;
use std::path::Path;

/// Build the existing short Git status and staged/unstaged diff report.
///
/// # Arguments
/// * `cwd` — Repository working directory.
///
/// # Returns
/// Report text, a clean-tree message, or a status-query error message.
///
/// # Examples
/// ```no_run
/// let report = codetether_git_view::working_tree_report(std::path::Path::new("."));
/// assert!(!report.is_empty());
/// ```
pub fn working_tree_report(cwd: &Path) -> String {
    let status = match git(cwd, &["status", "--short"]) {
        Ok(out) => out,
        Err(error) => {
            return format!("Diff unavailable: {error}");
        }
    };
    let unstaged = git(cwd, &["diff", "--stat"]).unwrap_or_default();
    let staged = git(cwd, &["diff", "--cached", "--stat"]).unwrap_or_default();
    let text = if status.trim().is_empty() {
        "Working tree clean.".to_string()
    } else {
        format!(
            "Git status\n{}\n\nUnstaged diff\n{}\n\nStaged diff\n{}",
            super::report_text::fallback(&status),
            super::report_text::fallback(&unstaged),
            super::report_text::fallback(&staged),
        )
    };
    text
}
