use super::{font::Font, state::State, text::failure};
use anyhow::Result;
use windows_sys::Win32::{
    Foundation::HWND,
    UI::{HiDpi::GetDpiForWindow, WindowsAndMessaging::*},
};

pub(super) fn update(hwnd: HWND, state: &mut State) -> Result<()> {
    // SAFETY: layout runs only on the owning thread of the live window.
    let dpi = unsafe { GetDpiForWindow(hwnd) };
    anyhow::ensure!(dpi > 0, "Cannot determine window DPI");
    let font = Font::new(dpi)?;
    if let Some(controls) = &state.controls {
        for &(child, _) in &controls.children {
            // SAFETY: switch every child before releasing their previous font.
            unsafe {
                SendMessageW(child, WM_SETFONT, font.0 as usize, 1);
            }
        }
    }
    state.resources.font = Some(font);
    if let Some(controls) = &state.controls {
        for &(child, bounds) in &controls.children {
            let [x, y, width, height] = bounds.map(|v| (i64::from(v) * i64::from(dpi) / 96) as i32);
            // SAFETY: child is live; no z-order or activation change is requested.
            if unsafe {
                SetWindowPos(
                    child,
                    std::ptr::null_mut(),
                    x,
                    y,
                    width,
                    height,
                    SWP_NOZORDER | SWP_NOACTIVATE,
                )
            } == 0
            {
                return Err(failure("Cannot lay out local controls"));
            }
        }
    }
    Ok(())
}
