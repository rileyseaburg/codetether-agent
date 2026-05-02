//! Windows clipboard via the `windows` crate (raw-dylib linking).
//!
//! Uses the same linking strategy as the rest of our Win32 code to
//! avoid the MSVC conflict between `clipboard-win` (`kind="dylib"`)
//! and `windows` (`kind="raw-dylib"`) both referencing `user32.dll`.

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use windows::Win32::Foundation::{HANDLE, HGLOBAL};
use windows::Win32::System::DataExchange::*;
use windows::Win32::System::Memory::{GlobalAlloc, GlobalLock, GlobalSize, GlobalUnlock, GMEM_MOVEABLE};

const CF_UNICODETEXT: u32 = 13;

struct ClipboardGuard;
impl ClipboardGuard {
    fn open() -> Option<Self> {
        (0..5).find_map(|_| {
            unsafe { OpenClipboard(None) }.ok().map(|_| Self)
        })
    }
}
impl Drop for ClipboardGuard {
    fn drop(&mut self) { unsafe { let _ = CloseClipboard(); } }
}

/// Read plain text from the Windows clipboard.
pub fn get_clipboard_text() -> Option<String> {
    let _guard = ClipboardGuard::open()?;
    unsafe { IsClipboardFormatAvailable(CF_UNICODETEXT) }.ok()?;
    let handle = unsafe { GetClipboardData(CF_UNICODETEXT) }.ok()?;
    let hg = HGLOBAL(handle.0);
    let ptr = unsafe { GlobalLock(hg) };
    if ptr.is_null() { return None; }
    let len = unsafe { GlobalSize(hg) } / std::mem::size_of::<u16>();
    let slice = unsafe { std::slice::from_raw_parts(ptr as *const u16, len) };
    let text = OsString::from_wide(slice).to_string_lossy().trim_end_matches('\0').to_string();
    unsafe { let _ = GlobalUnlock(hg); }
    if text.is_empty() { None } else { Some(text) }
}

/// Write plain text to the Windows clipboard.
pub fn set_clipboard_text(text: &str) -> Option<()> {
    let _guard = ClipboardGuard::open()?;
    unsafe { EmptyClipboard() }.ok()?;
    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let byte_len = wide.len() * std::mem::size_of::<u16>();
    let hg = unsafe {
        let h = GlobalAlloc(GMEM_MOVEABLE, byte_len).ok()?;
        let ptr = GlobalLock(h);
        if ptr.is_null() { return None; }
        std::ptr::copy_nonoverlapping(wide.as_ptr() as *const u8, ptr as *mut u8, byte_len);
        let _ = GlobalUnlock(h);
        h
    };
    unsafe { SetClipboardData(CF_UNICODETEXT, Some(HANDLE(hg.0))) }.ok()?;
    Some(())
}
