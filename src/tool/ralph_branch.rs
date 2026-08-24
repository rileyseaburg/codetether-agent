//! Sandboxed current-branch lookup for Ralph orchestration.

use std::path::Path;

pub(super) fn current(dir: &Path) -> Option<String> {
    crate::tool::git::process::output_blocking_refs(
        dir,
        &["rev-parse", "--abbrev-ref", "HEAD"],
        false,
    )
    .ok()
    .filter(|output| output.status.success())
    .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_string())
}
