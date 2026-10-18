//! Deterministic provider preference and lexicographic fallback.

/// Apply the established choose default provider policy.
/// # Arguments
/// Receives the provider, request or error inputs indicated by the signature.
/// # Returns
/// The policy decision without invoking providers or mutating session state.
/// # Examples
/// ```
/// assert_eq!(codetether_provider_policy::choose_default_provider(&["zai", "openai"]), Some("openai"));
/// ```
pub fn choose_default_provider<'a>(providers: &'a [&'a str]) -> Option<&'a str> {
    let preferred = [
        "openai-codex",
        "openai",
        "anthropic",
        "github-copilot",
        "zai",
        "deepseek",
        "minimax",
        "openrouter",
        "novita",
        "moonshotai",
        "google",
    ];
    for name in preferred {
        if let Some(found) = providers.iter().copied().find(|p| *p == name) {
            return Some(found);
        }
    }
    providers.iter().copied().min()
}
