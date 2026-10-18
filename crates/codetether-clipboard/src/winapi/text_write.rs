//! Transfer a Unicode text allocation to the clipboard.
use super::guard::ClipboardGuard;
use windows::Win32::Foundation::{HANDLE, HGLOBAL};
use windows::Win32::System::DataExchange::*;
use windows::Win32::System::Memory::{GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalUnlock};

const CF_UNICODETEXT: u32 = 13;

/// Write plain text to the Windows clipboard.
/// # Arguments
/// * `text` - Text to transfer to the system clipboard.
/// # Returns
/// Some unit on success, or None when unavailable.
/// # Examples
/// ```no_run
/// let _result = codetether_clipboard::winapi::set_clipboard_text("hello");
/// ```
pub fn set_clipboard_text(text: &str) -> Option<()> {
    let _guard = ClipboardGuard::open()?;
    unsafe { EmptyClipboard() }.ok()?;
    let wide: Vec<u16> = text.encode_utf16().chain(std::iter::once(0)).collect();
    let byte_len = wide.len() * std::mem::size_of::<u16>();
    let hg = unsafe {
        let h = GlobalAlloc(GMEM_MOVEABLE, byte_len).ok()?;
        let ptr = GlobalLock(h);
        if ptr.is_null() {
            return None;
        }
        std::ptr::copy_nonoverlapping(wide.as_ptr() as *const u8, ptr as *mut u8, byte_len);
        let _ = GlobalUnlock(h);
        h
    };
    unsafe { SetClipboardData(CF_UNICODETEXT, Some(HANDLE(hg.0))) }.ok()?;
    Some(())
}
