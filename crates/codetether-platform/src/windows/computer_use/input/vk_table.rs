//! Virtual-key code lookup table for SendKeys named keys.

/// Resolve a SendKeys key name to a Windows virtual-key code.
///
/// Handles `{ENTER}`, `TAB`, bare letters, etc.
pub fn resolve_vk(name: &str) -> u16 {
    let upper = name
        .trim_start_matches('{')
        .trim_end_matches('}')
        .to_uppercase();
    match upper.as_str() {
        "ENTER" | "RETURN" | "~" => 0x0D,
        "TAB" => 0x09,
        "ESC" | "ESCAPE" => 0x1B,
        "BACKSPACE" | "BS" | "BKSP" => 0x08,
        "DELETE" | "DEL" => 0x2E,
        "INSERT" | "INS" => 0x2D,
        "HOME" => 0x24,
        "END" => 0x23,
        "PAGEUP" | "PGUP" => 0x21,
        "PAGEDOWN" | "PGDN" => 0x22,
        "UP" => 0x26,
        "DOWN" => 0x28,
        "LEFT" => 0x25,
        "RIGHT" => 0x27,
        "SPACE" => 0x20,
        "F1" => 0x70,
        "F2" => 0x71,
        "F3" => 0x72,
        "F4" => 0x73,
        "F5" => 0x74,
        "F6" => 0x75,
        "F7" => 0x76,
        "F8" => 0x77,
        "F9" => 0x78,
        "F10" => 0x79,
        "F11" => 0x7A,
        "F12" => 0x7B,
        "NUMPAD0" | "NUMPAD_0" | "KP0" => 0x60,
        "NUMPAD1" | "NUMPAD_1" | "KP1" => 0x61,
        "NUMPAD2" | "NUMPAD_2" | "KP2" => 0x62,
        "NUMPAD3" | "NUMPAD_3" | "KP3" => 0x63,
        "NUMPAD4" | "NUMPAD_4" | "KP4" => 0x64,
        "NUMPAD5" | "NUMPAD_5" | "KP5" => 0x65,
        "NUMPAD6" | "NUMPAD_6" | "KP6" => 0x66,
        "NUMPAD7" | "NUMPAD_7" | "KP7" => 0x67,
        "NUMPAD8" | "NUMPAD_8" | "KP8" => 0x68,
        "NUMPAD9" | "NUMPAD_9" | "KP9" => 0x69,
        "NUMPAD_DOT" | "NUMPADDOT" | "KP_DOT" => 0x6E,
        s if s.len() == 1 => s.chars().next().unwrap() as u16,
        _ => 0x0D, // fallback to VK_RETURN
    }
}
