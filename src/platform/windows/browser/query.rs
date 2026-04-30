//! Registry value reader for browser path extraction.

use std::path::PathBuf;

/// Read the default `(Default)` string value from an open registry key.
///
/// Returns `None` if the value is not a string or the path doesn't exist.
pub unsafe fn read_path_value(
    hkey: windows::Win32::System::Registry::HKEY,
) -> Option<PathBuf> {
    use windows::Win32::System::Registry::*;

    let mut buf = [0u16; 512];
    let mut len = (buf.len() * 2) as u32;
    let mut ty = 0u32;

    let result = RegQueryValueExW(
        hkey,
        windows::core::PCWSTR::null(),
        None,
        Some(&mut ty),
        Some(buf.as_mut_ptr() as *mut _),
        Some(&mut len),
    );
    if result.is_err() {
        return None;
    }

    let chars = (len / 2) as usize;
    let slice: Vec<u16> = buf[..chars].iter().copied().take_while(|&c| c != 0).collect();
    let s = String::from_utf16_lossy(&slice);
    let path = PathBuf::from(s.trim_end_matches('\0'));
    if path.is_file() { Some(path) } else { None }
}
