//! Capture `git diff --stat` for the TUI git view.

use std::path::Path;

/// Staged + unstaged diff stat summary.
pub fn capture_diff_stat(root: &Path) -> String {
    let Ok(output) = std::process::Command::new("git")
        .args(["-C", root.to_string_lossy().as_ref()])
        .args(["diff", "--stat"])
        .output()
    else {
        return "(unavailable)".into();
    };
    let stdout = String::from_utf8_lossy(&output.stdout);
    if stdout.trim().is_empty() {
        "Working tree clean.".into()
    } else {
        stdout.trim().to_string()
    }
}
