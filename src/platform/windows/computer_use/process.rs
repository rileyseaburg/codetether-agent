//! Process enumeration via ToolHelp32 — replaces PowerShell Get-Process.

use serde_json::{Value, json};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Diagnostics::ToolHelp::*;

/// List running processes.
///
/// Each returned object contains the process ID, parent process ID,
/// executable name, and thread count.
///
/// # Errors
///
/// Returns an error if `CreateToolhelp32Snapshot` fails.
pub fn list_processes() -> anyhow::Result<Vec<Value>> {
    unsafe { proc_inner() }
}

unsafe fn proc_inner() -> anyhow::Result<Vec<Value>> {
    let snap = unsafe { CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0) }?;
    let mut entry = PROCESSENTRY32W::default();
    entry.dwSize = std::mem::size_of::<PROCESSENTRY32W>() as u32;

    let mut procs = Vec::new();

    if unsafe { Process32FirstW(snap, &mut entry) }.is_ok() {
        loop {
            let name = String::from_utf16_lossy(
                &entry.szExeFile.iter().take_while(|&&c| c != 0).copied().collect::<Vec<u16>>()
            );
            procs.push(json!({
                "pid": entry.th32ProcessID,
                "ppid": entry.th32ParentProcessID,
                "name": name,
                "threads": entry.cntThreads,
            }));
            if unsafe { Process32NextW(snap, &mut entry) }.is_err() {
                break;
            }
        }
    }

    let _ = unsafe { CloseHandle(snap) };
    Ok(procs)
}
