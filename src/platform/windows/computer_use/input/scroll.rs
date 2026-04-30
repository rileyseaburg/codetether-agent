//! Scroll wheel via Win32 SendInput — replaces PowerShell mouse_event.

use windows::Win32::UI::Input::KeyboardAndMouse::*;

/// Send a vertical scroll event.
///
/// `amount` is in WHEEL_DELTA units (typically ±120).
///
/// # Errors
///
/// Returns an error if `SendInput` returns 0.
pub fn send_scroll(amount: i32) -> anyhow::Result<()> {
    unsafe { scroll_inner(amount) }
}

unsafe fn scroll_inner(amount: i32) -> anyhow::Result<()> {
    let scroll = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: amount as u32,
        dwFlags: MOUSEEVENTF_WHEEL,
        time: 0,
        dwExtraInfo: 0,
    };

    let input = [INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 { mi: scroll },
    }];

    let sent = unsafe { SendInput(&input, std::mem::size_of::<INPUT>() as i32) };
    anyhow::ensure!(sent == 1, "SendInput sent {sent}, expected 1");
    Ok(())
}
