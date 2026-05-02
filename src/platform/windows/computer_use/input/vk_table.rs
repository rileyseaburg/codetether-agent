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
        s if s.len() == 1 => s.chars().next().unwrap() as u16,
        _ => 0x0D, // fallback to VK_RETURN
    }
}
