//! Keyboard input via Win32 SendInput — replaces PowerShell SendKeys.

use windows::Win32::UI::Input::KeyboardAndMouse::*;

/// Press and release a single virtual-key code.
///
/// # Errors
///
/// Returns an error if `SendInput` fails.
pub fn send_key(vk: u16) -> anyhow::Result<()> {
    unsafe { key_inner(vk) }
}

unsafe fn key_inner(vk: u16) -> anyhow::Result<()> {
    let down = KEYBDINPUT {
        wVk: VIRTUAL_KEY(vk),
        wScan: 0,
        dwFlags: KEYBD_EVENT_FLAGS(0),
        time: 0,
        dwExtraInfo: 0,
    };
    let up = KEYBDINPUT {
        wVk: VIRTUAL_KEY(vk),
        wScan: 0,
        dwFlags: KEYEVENTF_KEYUP,
        time: 0,
        dwExtraInfo: 0,
    };

    let inputs = [
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: down } },
        INPUT { r#type: INPUT_KEYBOARD, Anonymous: INPUT_0 { ki: up } },
    ];

    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    anyhow::ensure!(sent == 2, "SendInput sent {sent}, expected 2");
    Ok(())
}
