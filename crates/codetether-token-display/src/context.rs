//! Determine context usage from the active model and current prompt.
use crate::TokenDisplay;
use crate::context_style::{label_for, style_for};
use codetether_telemetry::{ContextLimit, TOKEN_USAGE, TokenUsageSnapshot};
use ratatui::style::Style;

/// Compute the `(label, style)` for the context indicator, or `None` when no
/// model usage has been recorded yet.
/// # Arguments
/// * `display` - Model limit resolver.
/// * `snapshots` - Recorded usage by model.
/// # Returns
/// The current prompt's percentage label and severity style, when available.
/// # Examples
/// ```
/// let display = codetether_token_display::TokenDisplay::new();
/// assert!(codetether_token_display::context_indicator(&display, &[]).is_none());
/// ```
pub fn context_indicator(
    display: &TokenDisplay,
    snapshots: &[TokenUsageSnapshot],
) -> Option<(String, Style)> {
    let active = snapshots.iter().max_by_key(|s| s.totals.total())?;
    let limit = display.get_context_limit(&active.name)?;
    let used = TOKEN_USAGE
        .last_prompt_tokens_for(&active.name)
        .unwrap_or_else(|| active.totals.total().min(limit));
    let pct = ContextLimit::new(used, limit).percentage;
    Some((label_for(pct), style_for(pct)))
}
