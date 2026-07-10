//! Reasoning levels reported by the authenticated Codex model catalog.

/// Return supported wire-level efforts for a Codex model.
pub fn supported_levels(model: &str) -> &'static [&'static str] {
    let model = base_model(model);
    match model {
        "gpt-5.6-sol" | "gpt-5.6-terra" => &["low", "medium", "high", "xhigh", "max", "ultra"],
        "gpt-5.6-luna" => &["low", "medium", "high", "xhigh", "max"],
        "gpt-5.5"
        | "gpt-5.5-fast"
        | "gpt-5.4"
        | "gpt-5.4-mini"
        | "gpt-5.3-codex-spark"
        | "codex-auto-review" => &["low", "medium", "high", "xhigh"],
        _ => &[],
    }
}

/// Report whether a model accepts a reasoning-effort wire value.
pub fn supports(model: &str, effort: &str) -> bool {
    supported_levels(model).contains(&effort)
}

fn base_model(model: &str) -> &str {
    let model = model.rsplit('/').next().unwrap_or(model);
    model.split(':').next().unwrap_or(model)
}

#[cfg(test)]
mod tests {
    use super::supported_levels;

    #[test]
    fn catalog_distinguishes_ultra_and_max_models() {
        assert!(supported_levels("openai-codex/gpt-5.6-sol").contains(&"ultra"));
        assert!(!supported_levels("gpt-5.6-luna").contains(&"ultra"));
        assert_eq!(supported_levels("gpt-5.5").last(), Some(&"xhigh"));
    }
}
