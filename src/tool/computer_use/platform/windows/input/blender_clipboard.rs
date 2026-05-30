//! Clipboard paste helper for Blender console input.

use crate::platform::windows::computer_use::{parse_send_keys, send_chord};
use windows::Win32::Foundation::{HANDLE, HGLOBAL};
use windows::Win32::System::DataExchange::*;
use windows::Win32::System::Memory::{GMEM_MOVEABLE, GlobalAlloc, GlobalLock, GlobalUnlock};

const CF_UNICODETEXT: u32 = 13;

pub fn paste(text: &str) -> anyhow::Result<()> {
    set(text)?;
    send_chord(&parse_send_keys("^v"))
}

fn set(text: &str) -> anyhow::Result<()> {
    unsafe {
        OpenClipboard(None)?;
        EmptyClipboard()?;
        let wide: Vec<u16> = text.encode_utf16().chain([0]).collect();
        let bytes = wide.len() * std::mem::size_of::<u16>();
        let mem = GlobalAlloc(GMEM_MOVEABLE, bytes)?;
        copy_to_global(mem, &wide)?;
        SetClipboardData(CF_UNICODETEXT, Some(HANDLE(mem.0)))?;
        CloseClipboard()?;
    }
    Ok(())
}

unsafe fn copy_to_global(mem: HGLOBAL, wide: &[u16]) -> anyhow::Result<()> {
    let ptr = unsafe { GlobalLock(mem) };
    anyhow::ensure!(!ptr.is_null(), "GlobalLock returned null");
    unsafe { std::ptr::copy_nonoverlapping(wide.as_ptr(), ptr as *mut u16, wide.len()) };
    unsafe { GlobalUnlock(mem)? };
    Ok(())
}
