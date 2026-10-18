//! Explicit navigation key whitelist; no global modifier state.
use anyhow::{Result, bail};

pub(super) fn lookup(key: &str) -> Result<(u16, u8, bool)> {
    Ok(match key.to_ascii_lowercase().as_str() {
        "enter" | "return" => (0x0d, 0x1c, false),
        "tab" => (0x09, 0x0f, false),
        "escape" | "esc" => (0x1b, 0x01, false),
        "backspace" => (0x08, 0x0e, false),
        "space" => (0x20, 0x39, false),
        "left" | "arrowleft" => (0x25, 0x4b, true),
        "up" | "arrowup" => (0x26, 0x48, true),
        "right" | "arrowright" => (0x27, 0x4d, true),
        "down" | "arrowdown" => (0x28, 0x50, true),
        "home" => (0x24, 0x47, true),
        "end" => (0x23, 0x4f, true),
        "pageup" => (0x21, 0x49, true),
        "pagedown" => (0x22, 0x51, true),
        "insert" => (0x2d, 0x52, true),
        "delete" => (0x2e, 0x53, true),
        _ => bail!(
            "Shadow accepts simple navigation key names only; chords, modifiers, SendKeys syntax and printable keys are unsupported (use type_text)"
        ),
    })
}
pub(super) fn flags(scan: u8, extended: bool, up: bool) -> isize {
    (1u32
        | (u32::from(scan) << 16)
        | (u32::from(extended) << 24)
        | if up { 0xc000_0000 } else { 0 }) as isize
}
