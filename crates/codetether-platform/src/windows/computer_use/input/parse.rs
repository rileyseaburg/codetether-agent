//! SendKeys expression parser — converts `^c`, `%{TAB}`, `ENTER` to VK codes.

use super::vk_table::resolve_vk;

/// Parse a SendKeys expression into a sequence of virtual-key codes.
///
/// Supports:
/// - Bare keys: `ENTER`, `TAB`, `ESC`, `BACKSPACE`, etc.
/// - Modifiers: `^` (Ctrl), `%` (Alt), `+` (Shift)
/// - Combinations: `^c`, `+%{TAB}`, `^s`
/// - Special: `{ENTER}`, `{TAB}`, `{ESC}`
///
/// Returns one VK per element; callers must dispatch down/up for modifiers.
pub fn parse_send_keys(expr: &str) -> Vec<u16> {
    let expr = expr.trim();
    if expr.is_empty() {
        return vec![0x0D]; // VK_RETURN
    }
    if let Some(vks) = parse_word_chord(expr) {
        return vks;
    }

    // Collect modifier prefixes
    let mut mods = Vec::new();
    let rest = expr.trim_start_matches(['^', '%', '+'].as_ref());
    for ch in expr.chars() {
        match ch {
            '^' => mods.push(0x11), // VK_CONTROL
            '%' => mods.push(0x12), // VK_MENU (Alt)
            '+' => mods.push(0x10), // VK_SHIFT
            _ => break,
        }
    }

    let key = resolve_vk(rest);
    let mut result = mods.clone();
    result.push(key);
    // Caller (send_chord) handles modifier release automatically
    result
}

fn parse_word_chord(expr: &str) -> Option<Vec<u16>> {
    let parts = expr.split_whitespace().collect::<Vec<_>>();
    let (key, mods) = parts.split_last()?;
    if mods.is_empty() {
        return None;
    }
    let mut result = Vec::new();
    for part in mods {
        result.push(match part.to_ascii_uppercase().as_str() {
            "SHIFT" => 0x10,
            "CTRL" | "CONTROL" => 0x11,
            "ALT" => 0x12,
            _ => return None,
        });
    }
    result.push(resolve_vk(key));
    Some(result)
}
