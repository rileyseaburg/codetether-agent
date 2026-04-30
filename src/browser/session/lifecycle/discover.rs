//! Discover a Chromium-based browser executable on the host.
//!
//! `chromiumoxide` only speaks the Chrome DevTools Protocol, so we need a
//! Chromium-family browser: Chrome, Chromium, Edge, Brave, Opera, Vivaldi, or
//! any fork. This module probes every well-known install location per platform
//! and, on Windows, also consults `where.exe` as a last-resort lookup.

use std::path::PathBuf;

/// Return the first Chromium-family browser found on the host, or `None` if
/// no candidate location contains a usable executable.
pub(super) fn find_chromium_browser() -> Option<PathBuf> {
    if let Some(path) = candidate_paths().into_iter().find(|p| p.is_file()) {
        return Some(path);
    }
    platform_lookup()
}

#[cfg(target_os = "windows")]
fn candidate_paths() -> Vec<PathBuf> {
    const RELATIVE: &[&str] = &[
        // Google Chrome — all channels
        r"Google\Chrome\Application\chrome.exe",
        r"Google\Chrome Beta\Application\chrome.exe",
        r"Google\Chrome Dev\Application\chrome.exe",
        r"Google\Chrome SxS\Application\chrome.exe", // Canary
        // Microsoft Edge — all channels
        r"Microsoft\Edge\Application\msedge.exe",
        r"Microsoft\Edge Beta\Application\msedge.exe",
        r"Microsoft\Edge Dev\Application\msedge.exe",
        r"Microsoft\Edge SxS\Application\msedge.exe",
        // Other Chromium forks
        r"BraveSoftware\Brave-Browser\Application\brave.exe",
        r"BraveSoftware\Brave-Browser-Beta\Application\brave.exe",
        r"BraveSoftware\Brave-Browser-Nightly\Application\brave.exe",
        r"Vivaldi\Application\vivaldi.exe",
        r"Chromium\Application\chrome.exe",
    ];

    let mut bases: Vec<PathBuf> = Vec::new();
    for var in [
        "ProgramFiles",
        "ProgramFiles(x86)",
        "LocalAppData",
        "ProgramW6432",
    ] {
        if let Some(v) = std::env::var_os(var) {
            bases.push(PathBuf::from(v));
        }
    }

    let mut out = Vec::with_capacity(RELATIVE.len() * bases.len());
    for base in &bases {
        for rel in RELATIVE {
            out.push(base.join(rel));
        }
    }
    // Opera is intentionally omitted: its `launcher.exe` in the base folder is
    // a version-dispatch shim and does not reliably forward CDP flags. Opera
    // users can pass `executable_path` pointing at the versioned `opera.exe`.
    out
}

#[cfg(target_os = "macos")]
fn candidate_paths() -> Vec<PathBuf> {
    vec![
        PathBuf::from("/Applications/Google Chrome.app/Contents/MacOS/Google Chrome"),
        PathBuf::from("/Applications/Google Chrome Beta.app/Contents/MacOS/Google Chrome Beta"),
        PathBuf::from("/Applications/Google Chrome Canary.app/Contents/MacOS/Google Chrome Canary"),
        PathBuf::from("/Applications/Microsoft Edge.app/Contents/MacOS/Microsoft Edge"),
        PathBuf::from("/Applications/Brave Browser.app/Contents/MacOS/Brave Browser"),
        PathBuf::from("/Applications/Chromium.app/Contents/MacOS/Chromium"),
        PathBuf::from("/Applications/Vivaldi.app/Contents/MacOS/Vivaldi"),
    ]
}

#[cfg(all(unix, not(target_os = "macos")))]
fn candidate_paths() -> Vec<PathBuf> {
    vec![
        PathBuf::from("/usr/bin/google-chrome"),
        PathBuf::from("/usr/bin/google-chrome-stable"),
        PathBuf::from("/usr/bin/google-chrome-beta"),
        PathBuf::from("/usr/bin/google-chrome-unstable"),
        PathBuf::from("/usr/bin/chromium"),
        PathBuf::from("/usr/bin/chromium-browser"),
        PathBuf::from("/usr/bin/microsoft-edge"),
        PathBuf::from("/usr/bin/microsoft-edge-stable"),
        PathBuf::from("/usr/bin/microsoft-edge-beta"),
        PathBuf::from("/usr/bin/brave-browser"),
        PathBuf::from("/usr/bin/vivaldi"),
        PathBuf::from("/snap/bin/chromium"),
        PathBuf::from("/snap/bin/brave"),
    ]
}

/// Platform-specific fallback lookup using native Win32 APIs.
/// Uses registry probing + PATH lookup via the `platform` module,
/// eliminating the `where.exe` subprocess.
#[cfg(target_os = "windows")]
fn platform_lookup() -> Option<PathBuf> {
    crate::platform::windows::browser::find_browser()
}

#[cfg(not(target_os = "windows"))]
fn platform_lookup() -> Option<PathBuf> {
    None
}
