//! Always-on context-window usage indicator for the status bar.
//!
//! Surfaces how full the active model's context window is right now, based
//! on the **current turn's** prompt size (what the next request will send),
//! not cumulative lifetime tokens — the agent loop re-sends the growing
//! conversation each step and the RLM layer compresses history behind the
//! scenes, so a cumulative sum would show a bogus multi-thousand-percent
//! figure.

use ratatui::style::{Color, Modifier, Style};

use crate::telemetry::{ContextLimit, TOKEN_USAGE, TokenUsageSnapshot};
use crate::tui::token_display::TokenDisplay;

/// Compute the `(label, style)` for the context indicator, or `None` when no
/// model usage has been recorded yet.
pub(super) fn context_indicator(
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

fn label_for(pct: f64) -> String {
    let icon = if pct >= 90.0 {
        "🛑"
    } else if pct >= 75.0 {
        "⚠️"
    } else {
        "🧠"
    };
    format!("{icon} Context: {pct:.0}%")
}

fn style_for(pct: f64) -> Style {
    let color = if pct >= 90.0 {
        Color::Red
    } else if pct >= 75.0 {
        Color::Yellow
    } else if pct >= 50.0 {
        Color::Cyan
    } else {
        Color::Green
    };
    let base = Style::default().fg(color);
    if pct >= 75.0 {
        base.add_modifier(Modifier::BOLD)
    } else {
        base
    }
}
