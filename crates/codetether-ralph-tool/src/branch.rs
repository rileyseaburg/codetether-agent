//! Read the current branch using the original git invocation.

use std::process::Command;

/// Get the current git branch name
pub(crate) fn get_current_branch(dir: &std::path::Path) -> Option<String> {
    Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(dir)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        })
}
