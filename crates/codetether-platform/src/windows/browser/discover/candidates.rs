//! Probe known Chromium-family installation paths in environment order.
use std::path::PathBuf;

pub(super) fn candidate_paths() -> Option<PathBuf> {
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

    let bases: Vec<PathBuf> = [
        "ProgramFiles",
        "ProgramFiles(x86)",
        "LocalAppData",
        "ProgramW6432",
    ]
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
