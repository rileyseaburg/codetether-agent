//! GitHub CLI config discovery for the bash tool.
//!
//! This module finds an existing `gh` configuration directory so the CLI can
//! reuse host settings without probing the filesystem at runtime.
//!
//! # Examples
//!
//! ```ignore
//! let dir = resolve_gh_config_dir();
//! assert!(dir.is_none() || dir.unwrap().contains("gh"));
//! ```

use std::path::{Path, PathBuf};

/// Finds a reusable GitHub CLI config directory for authenticated commands.
///
/// The search prefers an explicit `GH_CONFIG_DIR`, then standard config paths,
/// and finally snap-managed directories when `gh` is installed via snap.
///
/// # Examples
///
/// ```ignore
/// let dir = resolve_gh_config_dir();
/// assert!(dir.is_some() || dir.is_none());
/// ```
pub(super) fn resolve_gh_config_dir() -> Option<String> {
    let explicit = std::env::var("GH_CONFIG_DIR").ok().map(PathBuf::from);
    if let Some(path) = explicit.filter(|path| has_gh_hosts_file(path)) {
        return Some(path.display().to_string());
    }
    let home = PathBuf::from(std::env::var_os("HOME")?);
    for path in [
        home.join(".config/gh"),
        home.join("snap/gh/current/.config/gh"),
    ] {
        if has_gh_hosts_file(&path) {
            return Some(path.display().to_string());
        }
    }
    for entry in std::fs::read_dir(home.join("snap/gh")).ok()?.flatten() {
        let path = entry.path().join(".config/gh");
        if has_gh_hosts_file(&path) {
            return Some(path.display().to_string());
        }
    }
    None
}

fn has_gh_hosts_file(path: &Path) -> bool {
    path.join("hosts.yml").exists()
}
