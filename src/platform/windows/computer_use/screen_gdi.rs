//! Shared GDI bitmap helpers for native capture.

use windows::Win32::Graphics::Gdi::*;

/// Build top-down 32-bit bitmap metadata for DIB extraction.
pub(in crate::platform::windows::computer_use) fn bitmap_info(
    width: i32,
    height: i32,
) -> BITMAPINFO {
    let mut bmi = BITMAPINFO::default();
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = width;
    bmi.bmiHeader.biHeight = -height;
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;
    bmi
}

/// Copy BGRA pixels from a GDI bitmap into `px`.
pub(in crate::platform::windows::computer_use) unsafe fn get_pixels(
    mem: HDC,
    bm: HBITMAP,
    height: u32,
    px: &mut [u8],
    bmi: &mut BITMAPINFO,
) -> i32 {
    unsafe {
        GetDIBits(
            mem,
            bm,
            0,
            height,
            Some(px.as_mut_ptr() as *mut _),
            bmi,
            DIB_RGB_COLORS,
        )
    }
}

/// Restore and release GDI resources held during capture.
pub(in crate::platform::windows::computer_use) unsafe fn cleanup(
    mem: HDC,
    bm: HBITMAP,
    old_bm: HGDIOBJ,
    hdc: HDC,
) {
    unsafe { SelectObject(mem, old_bm) };
    unsafe { DeleteObject(bm.into()) };
    unsafe { DeleteDC(mem) };
    unsafe { ReleaseDC(None, hdc) };
}
