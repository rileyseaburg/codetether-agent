//! Unified browser discovery — registry first, then candidate paths, then PATH.

use super::registry::registry_browser_path;
use std::path::PathBuf;
mod candidates;
use candidates::candidate_paths;

/// Find a Chromium-family browser using native Windows APIs.
///
/// Priority: registry lookup → well-known paths → PATH env.
pub fn find_browser() -> Option<PathBuf> {
    registry_browser_path()
        .or_else(candidate_paths)
        .or_else(path_lookup)
}

fn path_lookup() -> Option<PathBuf> {
    [
        "chrome.exe",
        "msedge.exe",
        "brave.exe",
        "vivaldi.exe",
        "chromium.exe",
    ]
    .iter()
    .find_map(|name| which::which(name).ok())
}
