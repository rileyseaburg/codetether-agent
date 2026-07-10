//! OpenAI Codex models that expose the Fast service tier.

const FAST_MODELS: &[&str] = &[
    "gpt-5.4",
    "gpt-5.5",
    "gpt-5.6-sol",
    "gpt-5.6-terra",
    "gpt-5.6-luna",
];

pub(crate) fn supports_fast(model: &str) -> bool {
    FAST_MODELS.contains(&base_model(model))
}

pub(super) fn parse_fast_alias(model: &str) -> Option<&str> {
    model
        .strip_suffix("-fast")
        .filter(|base| supports_fast(base))
}

fn base_model(model: &str) -> &str {
    let model = model.rsplit('/').next().unwrap_or(model);
    model.split(':').next().unwrap_or(model)
}

#[cfg(test)]
mod tests {
    use super::{parse_fast_alias, supports_fast};

    #[test]
    fn recognizes_new_codex_fast_models() {
        assert_eq!(parse_fast_alias("gpt-5.6-sol-fast"), Some("gpt-5.6-sol"));
        assert!(supports_fast("openai-codex/gpt-5.6-terra"));
        assert!(!supports_fast("openai-codex/gpt-5.3-codex"));
    }
}
