//! Dashboard and scrollable detail-panel widgets.
use ratatui::{
    Frame,
    layout::Rect,
    text::Line,
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Draw the dashboard using already-composed rows.
/// # Arguments
/// * `f` - Destination frame.
/// * `area` - Panel rectangle.
/// * `lines` - Composed dashboard rows.
/// # Returns
/// Draws a bordered dashboard panel.
/// # Examples
/// ```no_run
/// fn draw(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
///     codetether_agent_dashboard::render::dashboard(f, area, vec![]);
/// }
/// ```
pub fn dashboard(f: &mut Frame, area: Rect, lines: Vec<Line<'static>>) {
    let block = Block::default()
        .title(" Agents: managed children · tool agents · swarm workers ")
        .borders(Borders::ALL);
    f.render_widget(Paragraph::new(lines).block(block), area);
}

/// Draw a scrollable child transcript panel.
/// # Arguments
/// * `f` - Destination frame.
/// * `area` - Panel rectangle.
/// * `name` - Selected child name or the unknown fallback.
/// * `scroll` - Vertical scroll offset.
/// * `lines` - Composed detail rows.
/// # Returns
/// Draws an untrimmed, wrapped transcript with the selected child title.
/// # Examples
/// ```no_run
/// fn draw(f: &mut ratatui::Frame, area: ratatui::layout::Rect) {
///     codetether_agent_dashboard::render::detail(f, area, None, 0, vec![]);
/// }
/// ```
pub fn detail(
    f: &mut Frame,
    area: Rect,
    name: Option<&str>,
    scroll: u16,
    lines: Vec<Line<'static>>,
) {
    let name = name.unwrap_or("unknown");
    let block = Block::default()
        .title(format!(" Agent detail: @{name} "))
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(lines)
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));
    f.render_widget(paragraph, area);
}
