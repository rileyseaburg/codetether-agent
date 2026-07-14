//! Codex model availability and transport requirements.

const CHATGPT_MODELS: &[&str] = &[
    "gpt-5.5",
    "gpt-5.5-fast",
    "gpt-5.6-sol",
    "gpt-5.6-terra",
    "gpt-5.6-luna",
];

/// Models available through the ChatGPT Codex backend.
pub(super) fn chatgpt_models() -> &'static [&'static str] {
    CHATGPT_MODELS
}

/// Models that always require HTTP Responses streaming.
pub(super) fn requires_http(model: &str) -> bool {
    matches!(model, "gpt-5.6-sol" | "gpt-5.6-terra" | "gpt-5.6-luna")
}

#[cfg(test)]
mod tests {
    use super::{chatgpt_models, requires_http};

    #[test]
    fn classifies_chatgpt_transports() {
        assert!(chatgpt_models().contains(&"gpt-5.5"));
        assert!(!requires_http("gpt-5.5"));
        assert!(requires_http("gpt-5.6-sol"));
    }
}
