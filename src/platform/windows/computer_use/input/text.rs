//! Text input via Win32 SendInput — replaces PowerShell SendKeys.

/// Type a string by sending each character as a Unicode key event.
///
/// Uses `KEYEVENTF_UNICODE` scan codes so special chars like `+`, `^`, `%`
/// need no escaping (unlike PowerShell's `SendKeys`).
///
/// # Errors
///
/// Returns an error if any `SendInput` call fails.
pub fn send_text(text: &str) -> anyhow::Result<()> {
    for ch in text.chars() {
        send_unicode_char(ch)?;
    }
    Ok(())
}

fn send_unicode_char(ch: char) -> anyhow::Result<()> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    unsafe {
        let down = unicode_event(ch as u16, false);
        let up = unicode_event(ch as u16, true);
        let sent = SendInput(&[down, up], std::mem::size_of::<INPUT>() as i32);
        anyhow::ensure!(sent == 2, "SendInput sent {sent}, expected 2");
    }
    Ok(())
}

unsafe fn unicode_event(ch: u16, up: bool) -> INPUT {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    INPUT {
        r#type: INPUT_TYPE(1),
        Anonymous: INPUT_1 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: ch,
                dwFlags: if up { KEYEVENTF_KEYUP } else { KEYEVENTF_UNICODE }
                    | KEYEVENTF_SCANCODE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}
