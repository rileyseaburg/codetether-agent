//! Parse questions accepted by the goal answer-review flow.

pub(super) fn parse(input: &str) -> Option<&str> {
    if let Some(rest) = input.strip_prefix("/ask") {
        if !rest.starts_with(char::is_whitespace) {
            return None;
        }
        let question = rest.trim();
        if question.is_empty() {
            return None;
        }
        return Some(question);
    }
    if input.starts_with('/') || input.starts_with('!') || input.is_empty() {
        return None;
    }
    Some(input)
}
