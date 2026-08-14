//! Extra writable roots a Seatbelt-confined process needs beyond the policy.

use std::path::{Path, PathBuf};

/// `HOME` handed to sandboxed processes by the restricted environment.
///
/// It must be writable or tools that create dotfiles fail immediately. It is
/// listed explicitly because on macOS the per-user temp directory is under
/// `/var/folders`, not `/tmp`.
const SANDBOX_HOME: &str = "/tmp";

/// Roots to add to every profile, in addition to the policy's allowed paths.
pub(super) fn implicit(work_dir: &Path, temp_dir: &Path) -> Vec<PathBuf> {
    vec![
        PathBuf::from(SANDBOX_HOME),
        temp_dir.to_path_buf(),
        work_dir.to_path_buf(),
    ]
}

/// Resolve `path` through symlinks so SBPL `subpath` rules match.
///
/// macOS exposes `/tmp` and `/var` as symlinks into `/private`, and Seatbelt
/// evaluates rules against the resolved path. Unresolvable paths are returned
/// unchanged so a not-yet-created directory still yields a rule.
pub(super) fn resolved(path: &Path) -> Vec<PathBuf> {
    let canonical = std::fs::canonicalize(path).ok();
    match canonical {
        Some(resolved) if resolved != path => vec![path.to_path_buf(), resolved],
        _ => vec![path.to_path_buf()],
    }
}

#[cfg(test)]
#[path = "sandbox_seatbelt_roots_tests.rs"]
mod tests;