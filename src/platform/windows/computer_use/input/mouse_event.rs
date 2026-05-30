//! Send one native mouse button event.

use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn send_mouse_button(flags: MOUSE_EVENT_FLAGS, phase: &str) -> anyhow::Result<()> {
    let input = INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx: 0,
                dy: 0,
                mouseData: 0,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    };
    let sent = unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };
    anyhow::ensure!(sent == 1, "SendInput {phase} returned {sent}");
    Ok(())
}
