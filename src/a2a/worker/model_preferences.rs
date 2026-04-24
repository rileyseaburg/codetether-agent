//! Provider preference ordering for worker-selected models.
//!
//! The worker prefers OpenAI Codex first so autonomous execution defaults to
//! Codex GPT-5.5 when that provider is configured.
//!
//! # Examples
//!
//! ```ignore
//! let provider = choose_provider_for_tier(&["zai", "openai-codex"], Some("fast"));
//! assert_eq!(provider, "openai-codex");
//! ```

/// Chooses the preferred provider for the requested model tier.
///
/// OpenAI Codex is ranked ahead of other compatible providers so autonomous
/// tasks use Codex GPT-5.5 by default when the provider is configured.
///
/// # Examples
///
/// ```ignore
/// let provider = choose_provider_for_tier(&["zai", "openai-codex"], None);
/// assert_eq!(provider, "openai-codex");
/// ```
pub(super) fn choose_provider_for_tier<'a>(
    providers: &'a [&'a str],
    model_tier: Option<&str>,
) -> &'a str {
    provider_preferences_for_tier(model_tier)
        .iter()
        .find_map(|preferred| {
            providers
                .iter()
                .copied()
                .find(|provider| provider == preferred)
        })
        .or_else(|| {
            providers
                .iter()
                .copied()
                .find(|provider| *provider == "zai")
        })
        .unwrap_or_else(|| {
            assert!(!providers.is_empty(), "provider list must not be empty");
            providers[0]
        })
}

#[rustfmt::skip]
const FAST_PROVIDERS: &[&str] = &["openai-codex", "zai", "minimax-credits", "minimax", "moonshotai", "openrouter", "anthropic", "google", "openai", "github-copilot", "novita"];

#[rustfmt::skip]
const HEAVY_PROVIDERS: &[&str] = &["openai-codex", "zai", "minimax", "moonshotai", "openrouter", "anthropic", "google", "openai", "github-copilot", "novita"];

#[rustfmt::skip]
const BALANCED_PROVIDERS: &[&str] = &["openai-codex", "zai", "minimax", "minimax-credits", "moonshotai", "openrouter", "anthropic", "google", "openai", "github-copilot", "novita"];

fn provider_preferences_for_tier(model_tier: Option<&str>) -> &'static [&'static str] {
    match model_tier.unwrap_or("balanced") {
        "fast" | "quick" => FAST_PROVIDERS,
        "heavy" | "deep" => HEAVY_PROVIDERS,
        _ => BALANCED_PROVIDERS,
    }
}
