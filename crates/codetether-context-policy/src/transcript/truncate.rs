//! Unicode-scalar truncation shared by prompts and titles.

/// Truncates a string to max_chars and adds an ellipsis if needed.
pub fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(c) = chars.next() {
            output.push(c);
        } else {
            break;
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}
