//! Text input via Win32 SendInput — replaces PowerShell SendKeys.
//!
//! This module types arbitrary Unicode text into the active Windows desktop
//! target by calling the Win32 [`SendInput`] API directly. It is used by the
//! `computer_use` tool when an agent requests text entry through the native
//! desktop automation backend.
//!
//! Unlike `SendKeys`-style automation, this path sends UTF-16 scan units with
//! [`KEYEVENTF_UNICODE`]. That means characters such as `+`, `^`, `%`, braces,
//! and other SendKeys metacharacters are delivered as literal text rather than
//! being interpreted as shortcut syntax.
//!
//! # Preconditions
//!
//! The intended destination window must already have keyboard focus. This
//! module does not choose or focus a window; callers should use the platform
//! window-management actions before typing when focus is uncertain.
//!
//! # Side effects
//!
//! Functions in this module synthesize keyboard input at the OS level. The
//! generated text is delivered to whichever control currently receives keyboard
//! input.

use windows::Win32::UI::Input::KeyboardAndMouse::*;

/// Type a string by sending each character as a Unicode key event.
///
/// Uses `KEYEVENTF_UNICODE` so special chars like `+`, `^`, `%` need no
/// escaping (unlike PowerShell's `SendKeys`). Handles non-BMP characters
/// by emitting both UTF-16 surrogates.
///
/// # Arguments
///
/// * `text` - The exact Unicode string to type into the currently focused
///   desktop control.
///
/// # Returns
///
/// Returns `Ok(())` after every UTF-16 code unit in `text` has been sent as a
/// key-down/key-up pair.
///
/// # Errors
///
/// Returns an error if any `SendInput` call fails.
///
/// # Side effects
///
/// Sends synthetic keyboard input to the active Windows input target.
pub fn send_text(text: &str) -> anyhow::Result<()> {
    for ch in text.chars() {
        let mut utf16 = [0u16; 2];
        for unit in ch.encode_utf16(&mut utf16).iter().copied() {
            send_unicode_unit(unit)?;
        }
    }
    Ok(())
}

/// Sends one UTF-16 code unit as a Unicode key press and release.
///
/// This helper is responsible for turning a single UTF-16 unit into the pair of
/// Win32 input events expected by [`SendInput`]. Non-BMP characters are handled
/// by the caller as two separate surrogate units.
///
/// # Arguments
///
/// * `ch` - UTF-16 code unit to send through `KEYEVENTF_UNICODE`.
///
/// # Returns
///
/// Returns `Ok(())` when Win32 reports that both the key-down and key-up events
/// were accepted.
///
/// # Errors
///
/// Returns an error if [`SendInput`] reports that fewer than two events were
/// inserted into the input stream.
///
/// # Side effects
///
/// Injects a key-down/key-up pair into the Windows input queue.
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

/// Builds a Win32 keyboard input event for one Unicode UTF-16 code unit.
///
/// The returned [`INPUT`] uses `INPUT_KEYBOARD`, sets `wVk` to zero as required
/// for Unicode scan-code input, stores the UTF-16 code unit in `wScan`, and
/// always includes [`KEYEVENTF_UNICODE`]. When `up` is true the event also
/// includes [`KEYEVENTF_KEYUP`].
///
/// # Arguments
///
/// * `ch` - UTF-16 code unit to place in the `wScan` field.
/// * `up` - Whether to create the key-release event (`true`) or key-press event
///   (`false`).
///
/// # Returns
///
/// A fully populated Win32 [`INPUT`] structure suitable for [`SendInput`].
///
/// # Safety
///
/// This function is marked unsafe because it constructs Win32 union fields in
/// [`INPUT_0`]. Callers must ensure the `r#type` field and active union variant
/// remain consistent, as this function does by setting `r#type` to
/// [`INPUT_KEYBOARD`] and initializing the `ki` keyboard field.
unsafe fn unicode_event(ch: u16, up: bool) -> INPUT {
    use windows::Win32::UI::Input::KeyboardAndMouse::*;
    INPUT {
        r#type: INPUT_KEYBOARD,
        Anonymous: INPUT_0 {
            ki: KEYBDINPUT {
                wVk: VIRTUAL_KEY(0),
                wScan: ch,
                dwFlags: if up {
                    KEYEVENTF_KEYUP
                } else {
                    KEYBD_EVENT_FLAGS(0)
                } | KEYEVENTF_UNICODE,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}
