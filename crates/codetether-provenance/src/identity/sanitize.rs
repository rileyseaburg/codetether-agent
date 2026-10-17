//! Normalize identity text for Git names and email addresses.

pub(super) fn sanitize_identity(value: &str) -> Option<String> {
    let mut output = String::new();
    let mut last_was_dash = false;
    for ch in value.trim().chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            output.push(lower);
            last_was_dash = false;
        } else if !last_was_dash && !output.is_empty() {
            output.push('-');
            last_was_dash = true;
        }
        if output.len() >= 48 {
            break;
        }
    }
    while output.ends_with('-') {
        output.pop();
    }
    (!output.is_empty()).then_some(output)
}
