//! Modifier key helpers for mouse gestures.

use windows::Win32::UI::Input::KeyboardAndMouse::*;

pub fn modifier_vks(names: &[String]) -> anyhow::Result<Vec<u16>> {
    names.iter().map(|name| modifier_vk(name)).collect()
}

pub fn hold_modifiers(vks: &[u16]) -> anyhow::Result<()> {
    send_modifiers(vks.iter().copied(), false)
}

pub fn release_modifiers(vks: &[u16]) -> anyhow::Result<()> {
    send_modifiers(vks.iter().rev().copied(), true)
}

fn modifier_vk(name: &str) -> anyhow::Result<u16> {
    match name.to_ascii_lowercase().as_str() {
        "shift" => Ok(0x10),
        "ctrl" | "control" => Ok(0x11),
        "alt" => Ok(0x12),
        other => anyhow::bail!("unsupported modifier: {other}"),
    }
}

fn send_modifiers(keys: impl Iterator<Item = u16>, up: bool) -> anyhow::Result<()> {
    for vk in keys {
        let input = INPUT {
            r#type: INPUT_KEYBOARD,
            Anonymous: INPUT_0 {
                ki: KEYBDINPUT {
                    wVk: VIRTUAL_KEY(vk),
                    wScan: 0,
                    dwFlags: if up {
                        KEYEVENTF_KEYUP
                    } else {
                        KEYBD_EVENT_FLAGS(0)
                    },
                    time: 0,
                    dwExtraInfo: 0,
                },
            },
        };
        let sent = unsafe { SendInput(&[input], std::mem::size_of::<INPUT>() as i32) };
        anyhow::ensure!(sent == 1, "SendInput modifier returned {sent}");
    }
    Ok(())
}
