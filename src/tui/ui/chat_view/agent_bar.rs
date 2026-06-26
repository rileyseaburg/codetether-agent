//! Header tab bar listing the main agent and spawned sibling agents.
//!
//! Renders a single row of agent "tabs" so the user can manage many
//! agentic threads from one terminal. The active tab is highlighted;
//! Tab / Shift+Tab cycle the selection (see `agent_switch`).

use ratatui::{
    Frame,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::app::state::App;

/// Render the agent tab bar into `area` (no-op when `area` has zero height).
pub fn render_agent_bar(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    if area.height == 0 || app.state.spawned_agents.is_empty() {
        return;
    }
    let active = app.state.active_spawned_agent.as_deref();
    let mut spans: Vec<Span<'static>> = vec![agent_tab("main", active.is_none(), false)];
    let mut names: Vec<&String> = app.state.spawned_agents.keys().collect();
    names.sort();
    for name in names {
        let processing = app
            .state
            .spawned_agents
            .get(name)
            .map(|a| a.is_processing)
            .unwrap_or(false);
        spans.push(Span::raw(" "));
        spans.push(agent_tab(name, active == Some(name.as_str()), processing));
    }
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}

/// Build a single tab span: `[ name ⋯ ]`, highlighted when selected.
fn agent_tab(name: &str, selected: bool, processing: bool) -> Span<'static> {
    let marker = if processing { " ⋯" } else { "" };
    let label = format!(" {name}{marker} ");
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
