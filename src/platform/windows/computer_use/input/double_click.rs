//! Double-click via Win32 SendInput.

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

/// Move cursor to (x, y) and perform a double left click.
pub fn send_double_click(x: i32, y: i32) -> anyhow::Result<()> {
    unsafe { dbl_click_inner(x, y) }
}

unsafe fn dbl_click_inner(x: i32, y: i32) -> anyhow::Result<()> {
    unsafe { SetCursorPos(x, y) };
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
        INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 { mi: down },
        },
        INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 { mi: up },
        },
        INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 { mi: down },
        },
        INPUT {
            r#type: INPUT_MOUSE,
            Anonymous: INPUT_0 { mi: up },
        },
    ];
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    anyhow::ensure!(sent == 4, "SendInput returned {sent}, expected 4");
    Ok(())
}
