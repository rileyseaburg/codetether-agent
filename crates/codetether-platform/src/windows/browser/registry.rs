//! Windows Registry browser path lookup — replaces `where.exe` subprocess.
//!
//! Probes the Windows registry `App Paths` keys for Chromium-family browsers,
//! eliminating the need to shell out to `where.exe`.

mod probe;
use probe::probe_app_paths;
use std::path::PathBuf;

/// Executable names to look up under
/// `HKLM\SOFTWARE\Microsoft\Windows\CurrentVersion\App Paths\`.
const APP_PATHS: &[&str] = &[
    "chrome.exe",
    "msedge.exe",
    "brave.exe",
    "vivaldi.exe",
    "chromium.exe",
    "opera.exe",
];

/// Query the registry for a Chromium-family browser executable.
///
/// Checks `App Paths` under both `HKLM` and `HKCU`.
pub fn registry_browser_path() -> Option<PathBuf> {
    unsafe { probe_app_paths() }
}
