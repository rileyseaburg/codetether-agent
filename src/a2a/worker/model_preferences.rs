//! Provider preference ordering for worker-selected models.
//!
//! The worker prefers GLM and MiniMax families ahead of OpenAI defaults so the
//! autonomous execution path follows the repository's operational policy.
//!
//! # Examples
//!
//! ```ignore
//! let provider = choose_provider_for_tier(&["zai", "openai"], Some("fast"));
//! assert_eq!(provider, "zai");
//! ```

/// Chooses the preferred provider for the requested model tier.
///
/// GLM and MiniMax providers are ranked ahead of other compatible providers so
/// autonomous tasks stay on the preferred model families by default.
///
/// # Examples
///
/// ```ignore
/// let provider = choose_provider_for_tier(&["zai", "openai"], None);
/// assert_eq!(provider, "zai");
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
        .unwrap_or(providers[0])
}

fn provider_preferences_for_tier(model_tier: Option<&str>) -> &'static [&'static str] {
    match model_tier.unwrap_or("balanced") {
        "fast" | "quick" => &[
            "zai",
            "minimax-credits",
            "minimax",
            "moonshotai",
            "openrouter",
            "anthropic",
            "google",
            "openai",
            "github-copilot",
            "novita",
        ],
        "heavy" | "deep" => &[
            "zai",
            "minimax",
            "moonshotai",
            "openrouter",
            "anthropic",
            "google",
            "openai",
            "github-copilot",
            "novita",
        ],
        _ => &[
            "zai",
            "minimax",
            "minimax-credits",
            "moonshotai",
            "openrouter",
            "anthropic",
            "google",
            "openai",
            "github-copilot",
            "novita",
        ],
    }
}
