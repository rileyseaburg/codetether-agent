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
    None
}

/// Plain follow-ups belong to an already-open answer-review conversation.
pub(super) fn followup(input: &str) -> Option<&str> {
    if input.is_empty() || input.starts_with('/') || input.starts_with('!') {
        return None;
    }
    Some(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn ordinary_input_never_requests_an_answer_hold() {
        for input in [
            "Why?",
            "use the cloned repos",
            "stop",
            "continue",
            "",
            "/ask",
            "/asking why",
        ] {
            assert_eq!(super::parse(input), None);
        }
        assert_eq!(super::parse("/ask Why?"), Some("Why?"));
    }
}
