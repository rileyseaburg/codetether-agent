//! Capture `git branch -v` for the TUI git view.

use std::path::Path;

/// Local branches with last-commit info.
pub fn capture_branches(root: &Path) -> Vec<String> {
    let Ok(output) = std::process::Command::new("git")
        .args(["-C", root.to_string_lossy().as_ref()])
        .args(["branch", "-v"])
        .output()
    else {
        return vec![];
    };
    String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(String::from)
        .collect()
}
