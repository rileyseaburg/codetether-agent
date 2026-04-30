//! Native screen capture via GDI BitBlt — replaces PowerShell CopyFromScreen.

use super::encode::bgra_to_png;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::*;

/// Captures the full virtual screen as PNG bytes.
///
/// Returns `(png_bytes, width, height, virtual_x, virtual_y)`.
///
/// # Errors
///
/// Returns an error if GDI bitmap creation or BitBlt fails.
pub fn capture_screenshot() -> anyhow::Result<(Vec<u8>, u32, u32, i32, i32)> {
    unsafe { capture_inner() }
}

unsafe fn capture_inner() -> anyhow::Result<(Vec<u8>, u32, u32, i32, i32)> {
    let _ = SetProcessDPIAware();

    let width = GetSystemMetrics(SM_CXVIRTUALSCREEN);
    let height = GetSystemMetrics(SM_CYVIRTUALSCREEN);
    let x = GetSystemMetrics(SM_XVIRTUALSCREEN);
    let y = GetSystemMetrics(SM_YVIRTUALSCREEN);

    let hdc = GetDC(None);
    anyhow::ensure!(!hdc.is_invalid(), "GetDC failed");

    let mem = CreateCompatibleDC(Some(hdc));
    let bm = CreateCompatibleBitmap(hdc, width, height);
    let old_bm = SelectObject(mem, bm.into());
    let ok = BitBlt(mem, 0, 0, width, height, Some(hdc), x, y, SRCCOPY);
    if ok.is_err() {
        let _ = SelectObject(mem, old_bm);
        let _ = DeleteObject(bm.into());
        let _ = DeleteDC(mem);
        let _ = ReleaseDC(None, hdc);
        anyhow::bail!("BitBlt failed");
    }

    let mut bmi = BITMAPINFO::default();
    bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
    bmi.bmiHeader.biWidth = width;
    bmi.bmiHeader.biHeight = -height;
    bmi.bmiHeader.biPlanes = 1;
    bmi.bmiHeader.biBitCount = 32;

    let px_len = (width as usize * 4) * height as usize;
    let mut px = vec![0u8; px_len];
    let dib_ok = GetDIBits(mem, bm, 0, height as u32, Some(px.as_mut_ptr() as *mut _), &mut bmi, DIB_RGB_COLORS);

    // Always restore and clean up GDI resources
    let _ = SelectObject(mem, old_bm);
    let _ = DeleteObject(bm.into());
    let _ = DeleteDC(mem);
    let _ = ReleaseDC(None, hdc);

    anyhow::ensure!(dib_ok != 0, "GetDIBits failed");
    let png = bgra_to_png(width as u32, height as u32, px)?;
    Ok((png, width as u32, height as u32, x, y))
}
