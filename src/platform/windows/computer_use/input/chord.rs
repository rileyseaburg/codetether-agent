//! Modifier-chord keyboard input via Win32 SendInput.

use windows::Win32::UI::Input::KeyboardAndMouse::*;

/// Send a modifier chord (e.g. Ctrl+C) as a single SendInput batch.
///
/// `vks` is ordered: [modifier_down..., key, ...modifier_up].
/// This function sends all key-down events, then all key-up events,
/// allowing proper modifier + key combinations.
///
/// # Errors
///
/// Returns an error if SendInput fails.
pub fn send_chord(vks: &[u16]) -> anyhow::Result<()> {
    unsafe { chord_inner(vks) }
}

const MODIFIERS: &[u16] = &[0x10, 0x11, 0x12]; // SHIFT, CTRL, ALT

unsafe fn chord_inner(vks: &[u16]) -> anyhow::Result<()> {
    if vks.is_empty() {
        return Ok(());
    }
    // Build down+up events for each VK in order
    let mut inputs: Vec<INPUT> = Vec::with_capacity(vks.len() * 2);
    for &vk in vks {
        let is_mod = MODIFIERS.contains(&vk);
        let down = unsafe { make_kb_input(vk, false) };
        let up = unsafe { make_kb_input(vk, true) };
        if is_mod {
            inputs.push(down);
        } else {
            // Non-modifier key: down then up between modifiers
            inputs.push(down);
            inputs.push(up);
        }
    }
    // Release all held modifiers in reverse
    for &vk in vks.iter().rev() {
        if MODIFIERS.contains(&vk) {
            inputs.push(unsafe { make_kb_input(vk, true) });
        }
    }
    let sent = unsafe { SendInput(&inputs, std::mem::size_of::<INPUT>() as i32) };
    anyhow::ensure!(sent as usize == inputs.len(), "SendInput sent {sent}");
    Ok(())
}

unsafe fn make_kb_input(vk: u16, up: bool) -> INPUT {
    INPUT {
        r#type: INPUT_TYPE(1),
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
    }
}
