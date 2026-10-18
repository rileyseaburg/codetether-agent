use ratatui::{prelude::*, widgets::*};

use super::View;

/// Render the compact inspector in a bordered panel.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `state` - Borrowed session display values.
/// * `area` - Panel rectangle.
/// # Returns
/// Draws the inspector into the supplied frame area.
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// let area = f.area();
/// codetether_inspector_view::compact::render(f, &Default::default(), area);
/// # }
/// ```
pub fn render(f: &mut Frame, state: &View<'_>, area: Rect) {
    let block = Block::default().borders(Borders::ALL).title(" Inspector ");
    f.render_widget(Paragraph::new(super::build(state)).block(block), area);
}
