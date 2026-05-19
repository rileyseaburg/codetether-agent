//! Click-drag via Win32 SendInput.

use windows::Win32::UI::Input::KeyboardAndMouse::*;
use windows::Win32::UI::WindowsAndMessaging::SetCursorPos;

/// Move cursor to (x1, y1), depress left button, drag to (x2, y2), release.
pub fn send_drag(x1: i32, y1: i32, x2: i32, y2: i32) -> anyhow::Result<()> {
    unsafe { drag_inner(x1, y1, x2, y2) }
}

unsafe fn drag_inner(x1: i32, y1: i32, x2: i32, y2: i32) -> anyhow::Result<()> {
    unsafe { SetCursorPos(x1, y1) };
    let down = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTDOWN,
        time: 0,
        dwExtraInfo: 0,
    };
    let sent = unsafe {
        SendInput(
            &[INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 { mi: down },
            }],
            std::mem::size_of::<INPUT>() as i32,
        )
    };
    anyhow::ensure!(sent == 1, "SendInput down returned {sent}");

    std::thread::sleep(std::time::Duration::from_millis(50));

    unsafe { SetCursorPos(x2, y2) };
    std::thread::sleep(std::time::Duration::from_millis(50));

    let up = MOUSEINPUT {
        dx: 0,
        dy: 0,
        mouseData: 0,
        dwFlags: MOUSEEVENTF_LEFTUP,
        time: 0,
        dwExtraInfo: 0,
    };
    let sent = unsafe {
        SendInput(
            &[INPUT {
                r#type: INPUT_MOUSE,
                Anonymous: INPUT_0 { mi: up },
            }],
            std::mem::size_of::<INPUT>() as i32,
        )
    };
    anyhow::ensure!(sent == 1, "SendInput up returned {sent}");
    Ok(())
}
