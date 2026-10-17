//! Right-click via Win32 SendInput.

use super::move_cursor;
use windows::Win32::UI::Input::KeyboardAndMouse::*;

/// Move cursor to (x, y) and perform a right click.
pub fn send_right_click(x: i32, y: i32) -> anyhow::Result<()> {
    unsafe { right_click_inner(x, y) }
}

unsafe fn right_click_inner(x: i32, y: i32) -> anyhow::Result<()> {
    move_cursor(x, y)?;
    let down = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_RIGHTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };
    let up = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_RIGHTUP,
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
    ];
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    anyhow::ensure!(sent == 2, "SendInput returned {sent}, expected 2");
    Ok(())
}
