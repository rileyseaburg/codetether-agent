//! Recent-tool panel widgets and wrapping policy.
use ratatui::{
    layout::Rect,
    widgets::{Block, Borders, Paragraph, Wrap},
};
/// Render recent tool activity in a supplied rectangle.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `state` - Tool and processing values.
/// * `area` - Panel rectangle.
///
/// # Returns
/// Draws wrapped recent-tool rows.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// let view = codetether_inspector_view::InspectorView::default();
/// let area = f.area();
/// codetether_inspector_view::render_tool_history(f, &view, area);
/// # }
/// ```
pub fn render_tool_history(f: &mut ratatui::Frame, state: &crate::InspectorView<'_>, area: Rect) {
    let lines = crate::tool_lines::lines(state);
    f.render_widget(
        Paragraph::new(lines)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Tool History "),
            )
            .wrap(Wrap { trim: true }),
        area,
    );
}
