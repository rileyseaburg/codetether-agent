//! Require one exact occurrence before replacing a snippet.

pub(super) fn apply_exact_replace(
    content: &str,
    old_string: &str,
    new_string: &str,
    file: &str,
    index: usize,
) -> std::result::Result<String, String> {
    let count = content.matches(old_string).count();
    if count == 0 {
        let preview: String = old_string.chars().take(80).collect();
        return Err(format!(
            "edits[{index}] {file}: old_string not found. First 80 chars: \"{preview}\""
        ));
    }
    if count > 1 {
        return Err(format!(
            "edits[{index}] {file}: old_string found {count} times (must be unique). Add more context."
        ));
    }
    Ok(content.replacen(old_string, new_string, 1))
}
