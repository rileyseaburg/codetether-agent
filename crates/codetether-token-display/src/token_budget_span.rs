//! Context budget percentage badge.
use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Append a `ctx:NN%` badge built from the most recent
/// session context estimate.
/// No-op when no estimate has been observed for the session yet.
///
/// # Arguments
/// * `spans` - Destination for the context badge.
/// * `used` - Used context tokens.
/// * `budget` - Available context tokens.
///
/// # Returns
/// Appends a percentage badge when a nonzero estimate is available.
///
/// # Examples
/// ```
/// let mut spans=Vec::new();
/// codetether_token_display::token_spans::push_context_budget_span(&mut spans,Some(5),Some(10));
/// assert_eq!(spans.len(),2);
/// ```
pub fn push_context_budget_span(
    spans: &mut Vec<Span<'static>>,
    used: Option<usize>,
    budget: Option<usize>,
) {
    let (Some(used), Some(budget)) = (used, budget) else {
        return;
    };
    if budget == 0 {
        return;
    }
    let pct = ((used as f64 / budget as f64) * 100.0) as usize;
    let color = if pct > 90 {
        Color::Red
    } else if pct > 70 {
        Color::Yellow
    } else {
        Color::Green
    };
    spans.push(Span::raw(" | "));
    spans.push(Span::styled(
        format!("ctx:{pct}%"),
        Style::default().fg(color),
    ));
}
