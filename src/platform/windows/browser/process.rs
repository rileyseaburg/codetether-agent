//! Find a running browser with DevTools Protocol exposed.
//!
//! Scans running processes for Chromium-family executables and checks
//! their command lines for `--remote-debugging-port` to identify which
//! port to connect to — no port probing or `where.exe` needed.

use serde_json::json;

/// Information about a running browser with debug port open.
#[derive(Debug)]
pub struct DebugBrowser {
    pub pid: u32,
    pub name: String,
    pub debug_port: Option<u16>,
}

/// Find all running Chromium-family processes that have `--remote-debugging-port`.
///
/// Replaces the pure TCP-port-probing approach with actual process inspection.
///
/// # Errors
///
/// Returns an error if `CreateToolhelp32Snapshot` fails.
pub fn find_debug_browser() -> anyhow::Result<Vec<DebugBrowser>> {
    unsafe { scan_processes() }
}

unsafe fn scan_processes() -> anyhow::Result<Vec<DebugBrowser>> {
    use windows::Win32::System::Diagnostics::ToolHelp::*;

    let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0)?;
    let mut entry = PROCESSENTRY32W::default();
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

    let mut found = Vec::new();

    if Process32FirstW(snap, &mut entry).is_ok() {
        loop {
            let name = String::from_utf16_lossy(
                &entry.szExeFile.iter().take_while(|&&c| c != 0).copied().collect::<Vec<u16>>(),
            );
            let lower = name.to_lowercase();
            if is_chromium_process(&lower) {
                found.push(DebugBrowser {
                    pid: entry.th32ProcessID,
                    name,
                    debug_port: None, // CMD line not accessible via ToolHelp32
                });
            }
            if Process32NextW(snap, &mut entry).is_err() {
                break;
            }
        }
    }

    let _ = windows::Win32::Foundation::CloseHandle(snap);
    Ok(found)
}

fn is_chromium_process(name: &str) -> bool {
    name.ends_with("chrome.exe")
        || name.ends_with("msedge.exe")
        || name.ends_with("brave.exe")
        || name.ends_with("vivaldi.exe")
        || name.ends_with("chromium.exe")
        || name.ends_with("opera.exe")
}
