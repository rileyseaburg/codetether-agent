//! Repository access for git panel data.

use std::path::Path;
use std::process::Command;

use super::model::GitPanel;
use super::parse::parse_status;

/// Load git panel data from a repository path.
pub fn load_panel(root: &Path) -> Result<GitPanel, String> {
    let output = Command::new("git")
        .arg("status")
        .arg("--short")
        .arg("--branch")
        .current_dir(root)
        .output()
        .map_err(|err| format!("git status failed: {err}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    Ok(parse_status(&String::from_utf8_lossy(&output.stdout)))
}
