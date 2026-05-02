//! Windows Registry browser path lookup — replaces `where.exe` subprocess.
//!
//! Probes the Windows registry `App Paths` keys for Chromium-family browsers,
//! eliminating the need to shell out to `where.exe`.

use super::query;
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

unsafe fn probe_app_paths() -> Option<PathBuf> {
    use windows::Win32::System::Registry::*;
    use windows::core::PCWSTR;

    for name in APP_PATHS {
        let key_path = format!("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\App Paths\\{name}");
        let key_path_w: Vec<u16> = key_path.encode_utf16().chain(std::iter::once(0)).collect();

        let mut hkey = Default::default();
        if RegOpenKeyExW(
            HKEY_LOCAL_MACHINE,
            PCWSTR(key_path_w.as_ptr()),
            Some(0),
            KEY_READ,
            &mut hkey,
        )
        .is_err()
            && RegOpenKeyExW(
                HKEY_CURRENT_USER,
                PCWSTR(key_path_w.as_ptr()),
                Some(0),
                KEY_READ,
                &mut hkey,
            )
            .is_err()
        {
            continue;
        }

        let path = query::read_path_value(hkey);
        let _ = RegCloseKey(hkey);
        if path.is_some() {
            return path;
        }
    }
    None
}
