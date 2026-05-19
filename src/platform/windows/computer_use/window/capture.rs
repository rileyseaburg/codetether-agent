//! Capture a specific window as JPEG via Win32 PrintWindow/BitBlt.

use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;

/// Capture a window by HWND and encode as JPEG.
///
/// Returns `(jpeg_bytes, width, height)`.
pub fn capture_window_jpeg(hwnd: i64) -> anyhow::Result<(Vec<u8>, u32, u32)> {
    unsafe {
        let hwnd = HWND(hwnd as *mut _);
        let mut rect = std::mem::zeroed();
        GetWindowRect(hwnd, &mut rect)?;
        let w = (rect.right - rect.left).max(0) as u32;
        let h = (rect.bottom - rect.top).max(0) as u32;
        anyhow::ensure!(w > 0 && h > 0, "window has invalid dimensions ({w}x{h})");

        let hdc = GetWindowDC(Some(hwnd));
        anyhow::ensure!(!hdc.is_invalid(), "GetWindowDC failed");
        let mem = CreateCompatibleDC(Some(hdc));
        let bm = CreateCompatibleBitmap(hdc, w as i32, h as i32);
        let old = SelectObject(mem, bm.into());
        BitBlt(mem, 0, 0, w as i32, h as i32, Some(hdc), 0, 0, SRCCOPY);

        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w as i32,
                biHeight: -(h as i32),
                biPlanes: 1,
                biBitCount: 32,
                ..Default::default()
            },
            ..Default::default()
        };
        let px_len = (w as usize * 4) * h as usize;
        let mut px = vec![0u8; px_len];
        let ok = GetDIBits(
            mem,
            bm,
            0,
            h,
            Some(px.as_mut_ptr() as *mut _),
            &mut bmi,
            DIB_RGB_COLORS,
        );

        SelectObject(mem, old);
        DeleteObject(bm.into());
        DeleteDC(mem);
        ReleaseDC(Some(hwnd), hdc);
        anyhow::ensure!(ok != 0, "GetDIBits failed");

        let jpeg = super::super::encode::bgra_to_encoded(w, h, px, image::ImageFormat::Jpeg)?;
        Ok((jpeg, w, h))
    }
}
