//! Copy native Windows clipboard image bytes under a clipboard guard.
use super::{guard::ClipboardGuard, image_formats::Format};
use windows::Win32::Foundation::HGLOBAL;
use windows::Win32::System::DataExchange::{
    GetClipboardData, IsClipboardFormatAvailable, RegisterClipboardFormatW,
};
use windows::Win32::System::Memory::{GlobalLock, GlobalSize, GlobalUnlock};

/// Copy a native clipboard format without keeping it locked during decoding.
pub(super) fn read(format: Format) -> Option<Vec<u8>> {
    let format = match format {
        Format::Png => unsafe { RegisterClipboardFormatW(windows::core::w!("PNG")) },
        Format::DibV5 => 17,
        Format::Dib => 8,
    };
    if format == 0 {
        return None;
    }
    let _guard = ClipboardGuard::open()?;
    unsafe { IsClipboardFormatAvailable(format) }.ok()?;
    let handle = unsafe { GetClipboardData(format) }.ok()?;
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
