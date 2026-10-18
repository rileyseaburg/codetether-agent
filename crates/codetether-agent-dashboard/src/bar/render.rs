//! Ordered agent-header assembly and rendering.
use super::{
    Host, push_tool_agents, push_tui_agents, swarm,
    tab::{AgentTabMeta, agent_tab},
};
use ratatui::{Frame, text::Line, widgets::Paragraph};
/// Render the agent tab bar into `area` (no-op when empty and zero height).
/// # Arguments
/// Receives the frame, registry source and header rectangle.
/// # Returns
/// Draws ordered tabs when the header has height and agents are present.
/// # Examples
/// ```no_run
/// fn header<H: codetether_agent_dashboard::bar::Host>(f: &mut ratatui::Frame, host: &H) {
///     let area = f.area();
///     codetether_agent_dashboard::bar::render(f, host, area);
/// }
/// ```
pub fn render(f: &mut Frame, app: &impl Host, area: ratatui::layout::Rect) {
    let tool_agents = app.tool_agents();
    let has_tui = !app.agents().is_empty();
    let has_swarm = !app.subtasks().is_empty();
    if area.height == 0 || (!has_tui && !has_swarm && tool_agents.is_empty()) {
        return;
    }
    let active = app.active();
    let model = app.model();
    let mut spans = vec![agent_tab(AgentTabMeta {
        name: "main",
        model_id: model.as_deref(),
        session_id: app.session_id(),
        identity: None,
        indent: 0,
        selected: active.is_none(),
        processing: app.processing(),
    })];
    push_tui_agents(&mut spans, app.agents(), active);
    push_tool_agents(&mut spans, &tool_agents, app.agents(), active);
    swarm::push_swarm_agents(&mut spans, app.subtasks(), app.model(), app.active());
    f.render_widget(Paragraph::new(Line::from(spans)), area);
}
