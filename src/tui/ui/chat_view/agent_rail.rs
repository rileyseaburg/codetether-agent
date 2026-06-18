//! Per-agent status rail badges.
//!
//! Renders one compact badge per registered worker agent so the multi-agent
//! workload is visible at a glance. Driven by
//! `app.state.worker_bridge_registered_agents` (a `HashSet<String>`); returns
//! an empty `Vec` when no agents are registered.

use ratatui::{
    style::{Color, Modifier, Style},
    text::Span,
};

use crate::tui::app::state::App;

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
    let color = if processing {
        Color::Cyan
    } else {
        Color::DarkGray
    };
    let mut spans = Vec::new();
    for (i, name) in names.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw(" "));
        }
        spans.push(Span::styled(
            format!("◆ {name}"),
            Style::default().fg(color).add_modifier(Modifier::DIM),
        ));
    }
    spans
}
