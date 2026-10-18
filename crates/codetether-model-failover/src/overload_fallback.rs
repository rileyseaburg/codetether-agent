//! Same-provider temporary Codex model selection.

/// Select a lower-priority Codex family model for a temporary overload.
///
/// # Arguments
/// * `current` — current input.
/// * `available` — available input.
///
/// # Returns
/// The selected value when the policy permits it, otherwise None.
///
/// # Examples
/// ```
/// assert!(codetether_model_failover::codex_overload_fallback(None, &[]).is_none());
/// ```
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
