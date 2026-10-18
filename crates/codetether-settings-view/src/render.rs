//! Settings panel layout and host footer rendering.
use crate::{SettingsView, settings_lines};
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::{Block, Borders, Paragraph, Wrap},
};

/// Draw settings rows above the host's status footer.
/// # Arguments
/// * `f` - Target frame.
/// * `area` - Available screen rectangle.
/// * `settings` - Current settings values and labels.
/// * `status` - Status text passed to the footer.
/// * `render_status` - Application footer renderer.
/// # Returns
/// Unit after rendering the panel and invoking the footer once.
/// # Examples
/// ```no_run
/// # fn example(f: &mut ratatui::Frame, area: ratatui::layout::Rect, settings: codetether_settings_view::SettingsView) {
/// codetether_settings_view::render_settings(f, area, settings, "ready", |_, _, _| {});
/// # }
/// ```
pub fn render_settings(
    f: &mut Frame,
    area: Rect,
    settings: SettingsView,
    status: &str,
    render_status: impl FnOnce(&mut Frame, Rect, &str),
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Length(3)])
        .split(area);

    let widget = Paragraph::new(settings_lines(settings))
        .block(Block::default().borders(Borders::ALL).title("Settings"))
        .wrap(Wrap { trim: false });
    f.render_widget(widget, chunks[0]);
    render_status(f, chunks[1], status);
}
