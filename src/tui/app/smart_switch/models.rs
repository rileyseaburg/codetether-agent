//! Fallback model lists per provider.

/// Returns the ordered list of preferred fallback models for a given provider.
pub fn smart_switch_preferred_models(provider_name: &str) -> &'static [&'static str] {
    match provider_name {
        "minimax" => &["MiniMax-M3", "MiniMax-M2.5", "MiniMax-M2.1", "MiniMax-M2"],
        "minimax-credits" => &["MiniMax-M2.5-highspeed", "MiniMax-M2.1-highspeed"],
        "zai" | "zai-api" => &["glm-5", "glm-4.7", "glm-4.7-flash"],
        "openai-codex" => crate::provider::openai_codex::model_catalog::chatgpt_models(),
        "openrouter" => &["z-ai/glm-5:free", "z-ai/glm-5", "moonshotai/kimi-k2:free"],
        "github-copilot" | "github-copilot-enterprise" => &["gpt-5-mini", "gpt-4.1", "gpt-4o"],
        "openai" => &["gpt-4o-mini", "gpt-4.1"],
        "anthropic" => &["claude-sonnet-4-20250514", "claude-3-5-sonnet-20241022"],
        "google" => &["gemini-2.5-flash", "gemini-2.5-pro"],
        "gemini-web" => &["gemini-web-pro"],
        _ => &[],
    }
}

/// Select a lower-priority Codex family model for a temporary overload.
pub fn codex_overload_fallback(current: Option<&str>, available: &[String]) -> Option<String> {
    let current = current.unwrap_or_default();
    let suffix = current.rsplit('/').next().unwrap_or_default();
    let fast = suffix.contains("-fast");
    let effort = suffix
        .split_once(':')
        .map(|(_, effort)| format!(":{effort}"));
    ["gpt-5.6-terra", "gpt-5.6-luna"]
        .iter()
        .map(|base| {
            format!(
                "openai-codex/{base}{}{}",
                if fast { "-fast" } else { "" },
                effort.clone().unwrap_or_default()
            )
        })
        .find(|candidate| {
            !candidate.eq_ignore_ascii_case(current)
                && available
                    .iter()
                    .any(|model| model.eq_ignore_ascii_case(candidate))
        })
}

#[cfg(test)]
mod tests {
    use super::codex_overload_fallback;

    #[test]
    fn overload_fallback_preserves_fast_and_effort_variant() {
        let models = vec!["openai-codex/gpt-5.6-terra-fast:high".into()];
        assert_eq!(
            codex_overload_fallback(Some("openai-codex/gpt-5.6-sol-fast:high"), &models),
            Some("openai-codex/gpt-5.6-terra-fast:high".into())
        );
    }
}
