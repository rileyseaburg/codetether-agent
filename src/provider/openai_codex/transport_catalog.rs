//! Transport requirements for Codex models.

/// Models that always require HTTP Responses streaming.
pub(super) fn requires_http(model: &str) -> bool {
    matches!(model, "gpt-5.6-sol" | "gpt-5.6-terra" | "gpt-5.6-luna")
}

#[cfg(test)]
mod tests {
    use super::requires_http;

    #[test]
    fn classifies_chatgpt_transports() {
        assert!(!requires_http("gpt-5.5"));
        assert!(requires_http("gpt-5.6-sol"));
    }
}
