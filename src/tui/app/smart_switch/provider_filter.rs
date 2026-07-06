//! Filtering candidates by provider (used on account exhaustion).

use super::error_detection::normalize_provider_alias;

/// Remove every candidate that belongs to `exhausted_provider`.
///
/// Used when a provider account is exhausted (expired plan / billing) so
/// the smart-switch actually leaves that provider instead of trying its
/// other models.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::app::smart_switch::provider_filter::filter_out_provider;
///
/// let candidates = vec!["zai/glm-5".to_string(), "minimax/MiniMax-M3".to_string()];
/// let kept = filter_out_provider(candidates, Some("zai"));
/// assert_eq!(kept, vec!["minimax/MiniMax-M3".to_string()]);
/// ```
pub fn filter_out_provider(
    candidates: Vec<String>,
    exhausted_provider: Option<&str>,
) -> Vec<String> {
    let Some(exhausted) = exhausted_provider.map(normalize_provider_alias) else {
        return candidates;
    };
    candidates
        .into_iter()
        .filter(|c| {
            let provider = c.split('/').next().map(normalize_provider_alias);
            provider != Some(exhausted)
        })
        .collect()
}
