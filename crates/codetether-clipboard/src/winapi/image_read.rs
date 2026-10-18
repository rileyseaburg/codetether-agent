//! Copy raw clipboard DIB bytes under a clipboard guard.
use super::guard::ClipboardGuard;
use windows::Win32::Foundation::HGLOBAL;
use windows::Win32::System::DataExchange::{GetClipboardData, IsClipboardFormatAvailable};
use windows::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};

const CF_DIB: u32 = 8;

/// Copy the raw `CF_DIB` bytes off the clipboard.
pub(super) fn read_cf_dib() -> Option<Vec<u8>> {
    let _guard = ClipboardGuard::open()?;
    unsafe { IsClipboardFormatAvailable(CF_DIB) }.ok()?;
    let handle = unsafe { GetClipboardData(CF_DIB) }.ok()?;
    let hg = HGLOBAL(handle.0);
    let ptr = unsafe { GlobalLock(hg) };
    if ptr.is_null() {
        return None;
    }
    let len = unsafe { GlobalSize(hg) };
    let bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len) }.to_vec();
    unsafe {
        let _ = GlobalUnlock(hg);
    }
    (!bytes.is_empty()).then_some(bytes)
}
