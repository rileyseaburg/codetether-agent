//! Slash command hint matching for autocomplete preview.

use super::slash_hints::SLASH_HINTS;

/// Match a partial slash command to a hint string.
///
/// Exact matches get the full description; prefix matches get a list of
/// candidates; no match returns "Unknown command".
#[allow(dead_code)]
pub fn match_slash_command_hint(input: &str) -> String {
    let trimmed = input.trim_start();
    let input_lower = trimmed.to_lowercase();

    if let Some((cmd, desc)) = SLASH_HINTS.iter().find(|(cmd, _)| {
        let key = cmd.trim_end().to_ascii_lowercase();
        input_lower == key || input_lower.starts_with(&(key.clone() + " "))
    }) {
        return format!("{} — {}", cmd.trim(), desc);
    }

    let matches: Vec<_> = SLASH_HINTS
        .iter()
        .filter(|(cmd, _)| cmd.starts_with(&input_lower))
        .collect();

    if matches.len() == 1 {
        format!("{} — {}", matches[0].0.trim(), matches[0].1)
    } else if matches.is_empty() {
        "Unknown command".to_string()
    } else {
        let cmds: Vec<_> = matches.iter().map(|(cmd, _)| cmd.trim()).collect();
        cmds.join(" | ")
    }
}
