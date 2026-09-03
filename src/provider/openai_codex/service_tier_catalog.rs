//! OpenAI Codex models that expose the Fast service tier.

const FAST_MODELS: &[&str] = &[
    "gpt-6-astra",
    "gpt-reserve",
    "gpt-5.4",
    "gpt-5.5",
    "gpt-5.6-sol",
    "gpt-5.6-terra",
    "gpt-5.6-luna",
    "codex-auto-review",
];

/// Reports whether a model supports the priority-backed Fast alias.
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
        assert_eq!(parse_fast_alias("gpt-6-astra-fast"), Some("gpt-6-astra"));
        assert_eq!(parse_fast_alias("gpt-reserve-fast"), Some("gpt-reserve"));
        assert!(supports_fast("openai-codex/gpt-5.6-terra"));
        assert!(supports_fast("openai-codex/codex-auto-review"));
        assert!(!supports_fast("openai-codex/gpt-5.4-mini"));
        assert!(!supports_fast("openai-codex/gpt-5.3-codex"));
    }
}