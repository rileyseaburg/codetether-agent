//! Capture `git log --oneline` for the TUI git view.

use std::path::Path;

/// Last 20 commits as oneline strings.
pub fn capture_log(root: &Path) -> Vec<String> {
    let Ok(output) = std::process::Command::new("git")
        .args(["-C", root.to_string_lossy().as_ref()])
        .args(["log", "--oneline", "-20"])
        .output()
    else {
        return vec!["(git log unavailable)".into()];
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(String::from)
        .collect()
}
