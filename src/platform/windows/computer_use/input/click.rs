//! Mouse click via Win32 SendInput — replaces PowerShell mouse_event.

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

/// Move cursor to (x, y) and perform a left click.
///
/// # Errors
///
/// Returns an error if `SendInput` fails.
pub fn send_click(x: i32, y: i32) -> anyhow::Result<()> {
    unsafe { click_inner(x, y) }
}

unsafe fn click_inner(x: i32, y: i32) -> anyhow::Result<()> {
    // Move cursor
    let _ = SetCursorPos(x, y);

    // MOUSEINPUT for left-button down + up (absolute click)
    let down = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };
    let up = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTUP,
        time: 0,
        dwExtraInfo: 0,
    };

    let inputs = [
        INPUT { r#type: INPUT_MOUSE, Anonymous: INPUT_0 { mi: down } },
        INPUT { r#type: INPUT_MOUSE, Anonymous: INPUT_0 { mi: up } },
    ];

    let sent = SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    anyhow::ensure!(sent == 2, "SendInput returned {sent}, expected 2");
    Ok(())
}
