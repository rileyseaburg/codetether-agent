//! Text input via Win32 SendInput — replaces PowerShell SendKeys.

/// Type a string by sending each character as a Unicode key event.
///
/// Uses `KEYEVENTF_UNICODE` so special chars like `+`, `^`, `%` need no
/// escaping (unlike PowerShell's `SendKeys`). Handles non-BMP characters
/// by emitting both UTF-16 surrogates.
///
/// # Errors
///
/// Returns an error if any `SendInput` call fails.
pub fn send_text(text: &str) -> anyhow::Result<()> {
    let mut utf16 = Vec::new();
    for ch in text.chars() {
        utf16.clear();
        ch.encode_utf16(&mut utf16);
        for &unit in &utf16 {
            send_unicode_unit(unit)?;
        }
    }
    Ok(())
}

fn send_unicode_unit(ch: u16) -> anyhow::Result<()> {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    unsafe {
        let down = unicode_event(ch, false);
        let up = unicode_event(ch, true);
        let sent = SendInput(&[down, up], std::mem::size_of::<INPUT>() as i32);
        anyhow::ensure!(sent == 2, "SendInput sent {sent}, expected 2");
    }
    Ok(())
}

unsafe fn unicode_event(ch: u16, up: bool) -> INPUT {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: ch,
                dwFlags: if up { KEYEVENTF_KEYUP } else { KEYBD_EVENT_FLAGS(0) }
                    | KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}
