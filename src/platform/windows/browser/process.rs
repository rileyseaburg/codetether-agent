//! Find running Chromium-family browser processes.
//!
//! Scans running processes for Chromium-family executables via ToolHelp32.
//! Note: command-line inspection (for `--remote-debugging-port`) requires
//! additional OS APIs not available through ToolHelp32 alone.

/// Information about a running browser process.
#[derive(Debug)]
pub struct DebugBrowser {
    pub pid: u32,
    pub name: String,
    pub debug_port: Option<u16>,
}

/// Find all running Chromium-family processes.
///
/// Uses ToolHelp32 to enumerate processes by name. Command-line arguments
/// (including `--remote-debugging-port`) are not accessible via this API.
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
                &entry
                    .szExeFile
                    .iter()
                    .take_while(|&&c| c != 0)
                    .copied()
                    .collect::<Vec<u16>>(),
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
    const BROWSERS: &[&str] = &[
        "chrome.exe",
        "msedge.exe",
        "brave.exe",
        "vivaldi.exe",
        "chromium.exe",
        "opera.exe",
    ];
    BROWSERS.iter().any(|browser| name.ends_with(browser))
}
