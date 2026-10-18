//! Attachment success text with binary and truncation annotations.
pub(super) fn success(
    display_path: &str,
    truncated: bool,
    binary: bool,
    max_bytes: usize,
) -> String {
    let suffix = if binary {
        " (binary file metadata only)".to_string()
    } else if truncated {
        format!(" (truncated to {} bytes)", max_bytes)
    } else {
        String::new()
    };
    format!("Attached `{display_path}` to composer{suffix}. Press Enter to send.")
}
