//! Read Unicode text under a clipboard guard.
use super::guard::ClipboardGuard;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Foundation::HGLOBAL;
use windows::Win32::System::DataExchange::*;
use windows::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};

const CF_UNICODETEXT: u32 = 13;

/// Read plain text from the Windows clipboard.
/// # Arguments
/// None; reads the current process's system clipboard.
/// # Returns
/// Nonempty text, or None when unavailable.
/// # Examples
/// ```no_run
/// let _text = codetether_clipboard::winapi::get_clipboard_text();
/// ```
pub fn get_clipboard_text() -> Option<String> {
    let _guard = ClipboardGuard::open()?;
    unsafe { IsClipboardFormatAvailable(CF_UNICODETEXT) }.ok()?;
    let handle = unsafe { GetClipboardData(CF_UNICODETEXT) }.ok()?;
    let hg = HGLOBAL(handle.0);
    let ptr = unsafe { GlobalLock(hg) };
    if ptr.is_null() {
        return None;
    }
    let len = unsafe { GlobalSize(hg) } / std::mem::size_of::<u16>();
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u16, len) };
    let text = OsString::from_wide(slice)
        .to_string_lossy()
        .trim_end_matches('\0')
        .to_string();
    unsafe {
        let _ = GlobalUnlock(hg);
    }
    if text.is_empty() { None } else { Some(text) }
}
