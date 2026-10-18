//! Per-agent identity color in the worker-agent rail badges.
//!
//! Each registered agent's `◆ name` badge is tinted with its stable
//! identity color (see [`codetether_theme::agent_color`]).
//! Processing agents glow at full brightness; idle agents are dimmed.

use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

use codetether_theme::agent_color::agent_color;
use codetether_tui_style::gradient::rgb_supported;

/// Build a sorted worker-agent rail with stable identity colors.
///
/// # Arguments
/// * `names` - Registered worker names.
/// * `processing` - Whether workers are active.
///
/// # Returns
/// Owned diamond-prefixed agent spans.
///
/// # Examples
/// ```
/// let name = "worker".to_string();
/// assert_eq!(codetether_chat_status::agent_rail_spans(vec![&name], false).len(), 1);
/// ```
pub fn agent_rail_spans(mut names: Vec<&String>, processing: bool) -> Vec<Span<'static>> {
    names.sort();
    let rgb = rgb_supported();
    let mut spans = Vec::new();
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" "));
        }
        let color = agent_color(name, rgb);
        let mut style = Style::default().fg(color);
        if !processing {
            style = style.add_modifier(Modifier::DIM);
        }
        spans.push(Span::styled(format!("◆ {name}"), style));
    }
    spans
}
