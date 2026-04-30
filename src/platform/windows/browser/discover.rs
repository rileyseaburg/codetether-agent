//! Unified browser discovery — registry first, then candidate paths, then PATH.

use super::registry::registry_browser_path;
use std::path::PathBuf;

/// Find a Chromium-family browser using native Windows APIs.
///
/// Priority: registry lookup → well-known paths → PATH env.
pub fn find_browser() -> Option<PathBuf> {
    registry_browser_path()
        .or_else(candidate_paths)
        .or_else(path_lookup)
}

fn candidate_paths() -> Option<PathBuf> {
    const RELATIVE: &[&str] = &[
        r"Google\Chrome\Application\chrome.exe",
        r"Google\Chrome Beta\Application\chrome.exe",
        r"Google\Chrome Dev\Application\chrome.exe",
        r"Google\Chrome SxS\Application\chrome.exe",
        r"Microsoft\Edge\Application\msedge.exe",
        r"Microsoft\Edge Beta\Application\msedge.exe",
        r"Microsoft\Edge Dev\Application\msedge.exe",
        r"Microsoft\Edge SxS\Application\msedge.exe",
        r"BraveSoftware\Brave-Browser\Application\brave.exe",
        r"Vivaldi\Application\vivaldi.exe",
        r"Chromium\Application\chrome.exe",
    ];

    let bases: Vec<PathBuf> = ["ProgramFiles", "ProgramFiles(x86)", "LocalAppData", "ProgramW6432"]
        .iter()
        .filter_map(|v| std::env::var_os(v).map(PathBuf::from))
        .collect();

    for base in &bases {
        for rel in RELATIVE {
            let p = base.join(rel);
            if p.is_file() {
                return Some(p);
            }
        }
    }
    None
}

fn path_lookup() -> Option<PathBuf> {
    ["chrome.exe", "msedge.exe", "brave.exe", "vivaldi.exe", "chromium.exe"]
        .iter()
        .find_map(|name| which::which(name).ok())
}
