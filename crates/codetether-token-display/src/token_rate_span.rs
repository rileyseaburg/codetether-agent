//! Conditional live token-rate badge.
use ratatui::{
    style::{Color, Style},
    text::Span,
};

/// Append a `⚡N tok/s` live throughput badge while a turn is streaming.
/// Yellow under 10 tok/s (likely stalled), cyan above. No-op when idle.
///
/// # Arguments
/// * `spans` - Destination for the rate badge.
/// * `throughput` - Latest token rate.
/// * `processing` - Whether the turn is active.
///
/// # Returns
/// Appends a rate badge only for an active turn with a sample.
///
/// # Examples
/// ```
/// let mut spans=Vec::new();
/// codetether_token_display::token_spans::push_throughput_span(&mut spans,Some(20.0),true);
/// assert_eq!(spans.len(),2);
/// ```
pub fn push_throughput_span(
    spans: &mut Vec<Span<'static>>,
    throughput: Option<f64>,
    processing: bool,
) {
    let Some(tps) = throughput else {
        return;
    };
    if !processing {
        return;
    }
    spans.push(Span::raw(" | "));
    let color = if tps < 10.0 {
        Color::Yellow
    } else {
        Color::Cyan
    };
    spans.push(Span::styled(
        format!("⚡{:.0} tok/s", tps),
        Style::default().fg(color),
    ));
}
