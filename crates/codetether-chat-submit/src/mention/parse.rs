/// Extract an @mention target from input text.
/// Returns `Some((agent_name, message))` if the input starts with `@`
/// followed by a non-empty name, a space, and a non-empty message.
pub fn parse_mention(input: &str) -> Option<(String, String)> {
    let trimmed = input.trim();
    if !trimmed.starts_with('@') {
        return None;
    }
    let rest = &trimmed[1..];
    let space_pos = rest.find(' ')?;
    let name = rest[..space_pos].to_string();
    let msg = rest[space_pos..].trim().to_string();
    if name.is_empty() || msg.is_empty() {
        return None;
    }
    Some((name, msg))
}
