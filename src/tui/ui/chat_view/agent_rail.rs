//! Per-agent identity color in the worker-agent rail badges.
//!
//! Each registered agent's `◆ name` badge is tinted with its stable
//! identity color (see [`crate::tui::ui::agent_color`]).
//! Processing agents glow at full brightness; idle agents are dimmed.

use ratatui::{
    style::{Modifier, Style},
    text::Span,
};

use crate::tui::app::state::App;
use crate::tui::ui::agent_color::agent_color;
use crate::tui::ui::gradient::rgb_supported;

/// Build one diamond-prefixed badge per registered worker agent.
///
/// # Examples
///
/// ```rust,no_run
/// use codetether_agent::tui::ui::chat_view::agent_rail::agent_rail_spans;
/// # fn demo(app: &codetether_agent::tui::app::state::App) {
/// let spans = agent_rail_spans(app);
/// let _ = spans.len();
/// # }
/// ```
pub fn agent_rail_spans(app: &App) -> Vec<Span<'static>> {
    let mut names: Vec<&String> = app.state.worker_bridge_registered_agents.iter().collect();
    names.sort();
    let processing = app
        .state
        .worker_bridge_processing_state
        .unwrap_or(app.state.processing);
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
