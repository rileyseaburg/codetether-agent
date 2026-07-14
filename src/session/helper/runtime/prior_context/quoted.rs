//! Removal of quoted examples before directive classification.

/// Remove double-quoted and backtick-delimited spans from user prose.
pub(super) fn strip(input: &str) -> String {
    let mut output = String::with_capacity(input.len());
    let mut quoted = String::new();
    let mut closing = None;
    for character in input.chars() {
        if let Some(expected) = closing {
            if character == expected {
                if target_only(&quoted) {
                    output.push_str(&quoted);
                }
                closing = None;
                output.push(' ');
                quoted.clear();
            } else {
                quoted.push(character);
            }
            continue;
        }
        closing = match character {
            '"' => Some('"'),
            '“' => Some('”'),
            '`' => Some('`'),
            _ => {
                output.push(character);
                None
            }
        };
    }
    if closing.is_some() && target_only(&quoted) {
        output.push_str(&quoted);
    }
    output
}

fn target_only(input: &str) -> bool {
    let normalized = input
        .to_lowercase()
        .replace(['_', '-'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    super::targets::PHRASES.contains(&normalized.as_str())
        || ["memory", "scope", "sessions"].contains(&normalized.as_str())
}
