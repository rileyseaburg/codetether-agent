use crate::TypingError;
use windows_sys::Win32::UI::Input::KeyboardAndMouse::*;
use zeroize::Zeroizing;

pub(super) fn quiet() -> Result<(), TypingError> {
    for key in [
        VK_SHIFT, VK_CONTROL, VK_MENU, VK_LWIN, VK_RWIN, VK_ESCAPE, VK_LBUTTON, VK_RBUTTON,
    ] {
        // SAFETY: checks only cancel/modifier state; never records keyboard input.
        if unsafe { GetAsyncKeyState(i32::from(key)) } < 0 {
            return Err(TypingError::TargetChanged);
        }
    }
    Ok(())
}
fn event(unit: u16, release: bool) -> INPUT {
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: 0,
                wScan: unit,
                dwFlags: KEYEVENTF_UNICODE | if release { KEYEVENTF_KEYUP } else { 0 },
                time: 0,
                dwExtraInfo: 0x43545459,
            },
        },
    }
}
pub(super) fn character(character: char) -> Result<(), TypingError> {
    let mut utf16 = Zeroizing::new([0u16; 2]);
    let units = character.encode_utf16(&mut *utf16);
    let mut events = [INPUT::default(); 4];
    for (index, unit) in units.iter().enumerate() {
        events[index * 2] = event(*unit, false);
        events[index * 2 + 1] = event(*unit, true);
    }
    let length = units.len() * 2;
    // SAFETY: bounded array of Unicode keyboard events; no virtual keys or mouse input.
    let sent = unsafe {
        SendInput(
            length as u32,
            events.as_ptr(),
            std::mem::size_of::<INPUT>() as i32,
        )
    } as usize;
    if sent < length && sent % 2 == 1 {
        let release = event(units[(sent - 1) / 2], true);
        // SAFETY: releases only our last inserted Unicode key; never retries text.
        unsafe {
            SendInput(1, &release, std::mem::size_of::<INPUT>() as i32);
        }
    }
    if sent == length {
        Ok(())
    } else {
        Err(TypingError::InputRejected)
    }
}
