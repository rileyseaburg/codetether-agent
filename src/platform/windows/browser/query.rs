//! Registry value reader for browser path extraction.

use std::path::PathBuf;

/// Read the default `(Default)` string value from an open registry key.
///
/// Queries the required buffer size first to avoid truncation,
/// then verifies the value is `REG_SZ` before reading.
pub unsafe fn read_path_value(
    hkey: windows::Win32::System::Registry::HKEY,
) -> Option<PathBuf> {
    use windows::Win32::System::Registry::*;

    let mut len = 0u32;
    let mut ty = REG_NONE;
    if RegQueryValueExW(hkey, windows::core::PCWSTR::null(), None, Some(&mut ty), None, Some(&mut len)).is_err() {
        return None;
    }
    if ty != REG_SZ {
        return None;
    }

    let mut buf: Vec<u16> = vec![0; len as usize / std::mem::size_of::<u16>()];
    if RegQueryValueExW(
        hkey, windows::core::PCWSTR::null(), None, Some(&mut ty),
        Some(buf.as_mut_ptr() as *mut _), Some(&mut len),
    ).is_err() {
        return None;
    }

    let s = String::from_utf16_lossy(&buf[..len as usize / std::mem::size_of::<u16>()]);
    let path = PathBuf::from(s.trim_end_matches('\0'));
    if path.is_file() { Some(path) } else { None }
}
