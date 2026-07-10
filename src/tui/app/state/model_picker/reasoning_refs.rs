//! OpenAI Codex reasoning variants shown by the TUI model picker.

use crate::provider::openai_codex::{reasoning_catalog, service_tier_catalog};

pub(super) fn expand(provider: &str, model_ref: String) -> Vec<String> {
    if provider != "openai-codex" {
        return vec![model_ref];
    }
    let levels = reasoning_catalog::supported_levels(&model_ref);
    let mut variants = Vec::with_capacity((levels.len() + 1) * 2);
    append_family(&mut variants, &model_ref, levels);
    if service_tier_catalog::supports_fast(&model_ref) {
        append_family(&mut variants, &format!("{model_ref}-fast"), levels);
    }
    variants
}

fn append_family(variants: &mut Vec<String>, model: &str, levels: &[&str]) {
    variants.push(model.to_string());
    variants.extend(levels.iter().map(|level| format!("{model}:{level}")));
}

#[cfg(test)]
mod tests {
    use super::expand;

    #[test]
    fn sol_includes_ultra_while_luna_stops_at_max() {
        let sol = expand("openai-codex", "openai-codex/gpt-5.6-sol".into());
        let luna = expand("openai-codex", "openai-codex/gpt-5.6-luna".into());
        assert!(sol.contains(&"openai-codex/gpt-5.6-sol".into()));
        assert!(sol.contains(&"openai-codex/gpt-5.6-sol:ultra".into()));
        assert!(sol.contains(&"openai-codex/gpt-5.6-sol-fast:ultra".into()));
        assert!(!luna.iter().any(|model| model.ends_with(":ultra")));
        assert!(luna.iter().any(|model| model.ends_with(":max")));
    }

    #[test]
    fn other_providers_are_unchanged() {
        let model = "openrouter/openai/gpt-5.6-sol".to_string();
        assert_eq!(expand("openrouter", model.clone()), vec![model]);
    }
}
