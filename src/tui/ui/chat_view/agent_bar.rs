//! Header tab bar showing the spawned-agent tree (main + sibling agents).
//!
//! Renders a single row of agent "tabs" in depth-first order so the user can
//! manage many agentic threads — including agents that spawned agents — from
//! one terminal. Nesting depth is shown via leading dots; the active tab is
//! highlighted and a busy agent shows a `⋯` marker. Tab / Shift+Tab cycle the
//! selection (see `agent_focus`).

use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::app::state::App;
use crate::tui::app::state::agent_tree::dfs_order;

/// Render the agent tab bar into `area` (no-op when `area` has zero height).
pub fn render_agent_bar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if area.height == 0 || app.state.spawned_agents.is_empty() {
        return;
    }
    let active = app.state.active_spawned_agent.as_deref();
    let mut spans: Vec<Span<'static>> = vec![agent_tab("main", 0, active.is_none(), false)];
    for node in dfs_order(&app.state.spawned_agents) {
        let agent = app.state.spawned_agents.get(&node.name);
        let processing = agent.map(|a| a.is_processing).unwrap_or(false);
        spans.push(Span::raw(" "));
        spans.push(agent_tab(
            &node.name,
            node.depth + 1,
            active == Some(node.name.as_str()),
            processing,
        ));
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

/// Build a single tab span: `·· name ⋯`, indented by `indent` dot pairs and
/// highlighted when selected.
fn agent_tab(name: &str, indent: u8, selected: bool, processing: bool) -> Span<'static> {
    let dots = "·".repeat(indent as usize);
    let marker = if processing { " ⋯" } else { "" };
    let label = format!(" {dots}{name}{marker} ");
    if selected {
        Span::styled(
            label,
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
    } else {
        Span::styled(label, Style::default().fg(Color::Gray))
    }
}
