//! User input textbox renderer.
//!
//! Renders a bordered [`Paragraph`] with a context-sensitive title.
//! Delegates cursor placement to [`place_cursor`].

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use super::cursor::place_cursor;
use crate::ComposerView;
use codetether_theme::ColorPalette;

use codetether_tui_style::gradient::rgb_supported;
use codetether_tui_style::spinner::spinner_color;

/// Paint the composer textbox and terminal cursor.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `view` - Borrowed input and display state.
/// * `area` - Input panel rectangle.
/// * `palette` - Default border palette.
/// * `suffix` - Title suffix, such as attachment information.
///
/// # Returns
/// Draws the bordered input and places its cursor.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// let view = codetether_composer_view::ComposerView { text: "hello", cursor: 5, processing: false, command_mode: false };
/// let area = f.area();
/// codetether_composer_view::render_input(f, &view, area, &codetether_theme::ColorPalette::marketing(), "");
/// # }
/// ```
pub fn render_input(
    f: &mut Frame,
    view: &ComposerView<'_>,
    area: Rect,
    palette: &ColorPalette,
    suffix: &str,
) {
    let title = super::input_title::build(view, suffix, area.width);
    let border_color = if view.processing {
        // Pulse with the live neon hue while the agent works.
        if rgb_supported() {
            spinner_color()
        } else {
            Color::Yellow
        }
    } else if view.command_mode {
        Color::Magenta
    } else {
        palette.border
    };
    let input = Paragraph::new(view.text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(title),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(input, area);
    place_cursor(f, view, area);
}
