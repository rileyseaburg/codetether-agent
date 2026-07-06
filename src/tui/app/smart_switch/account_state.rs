//! Detection of provider account-level exhaustion (vs per-model errors).

/// Detects errors that exhaust the whole provider account, not just one model.
///
/// Examples: an expired subscription / coding plan, or an account-wide quota
/// cap. When true, retrying a *different model on the same provider* is futile,
/// so the smart-switch skips same-provider candidates and jumps straight to
/// other providers.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::app::smart_switch::account_state::is_provider_account_exhausted;
///
/// assert!(is_provider_account_exhausted(
///     "GLM Coding Plan package has expired and is temporarily unavailable"
/// ));
/// assert!(!is_provider_account_exhausted("429 rate limit, try again"));
/// ```
pub fn is_provider_account_exhausted(err: &str) -> bool {
    let normalized = err.to_ascii_lowercase();
    let markers = [
        "expired",
        "subscription",
        "renew",
        "account is disabled",
        "account suspended",
        "billing",
        "insufficient balance",
        "insufficient_quota",
        "payment required",
    ];
    markers.iter().any(|m| normalized.contains(m))
}
