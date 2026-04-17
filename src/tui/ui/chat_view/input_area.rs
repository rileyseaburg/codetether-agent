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
use crate::tui::app::state::App;
use crate::tui::color_palette::ColorPalette;
use crate::tui::models::InputMode;

/// Draw the input textbox and place the terminal cursor.
///
/// # Examples
///
/// ```rust,no_run
/// # use codetether_agent::tui::ui::chat_view::input_area::render_input;
/// # fn d(f:&mut ratatui::Frame,a:&codetether_agent::tui::app::state::App){ let p=codetether_agent::tui::color_palette::ColorPalette::marketing(); render_input(f,a,ratatui::layout::Rect::new(0,0,40,3),&p,""); }
/// ```
pub fn render_input(f: &mut Frame, app: &App, area: Rect, palette: &ColorPalette, suffix: &str) {
    let title = if app.state.processing {
        format!(" Message (Processing — Enter to queue steering){suffix}")
    } else if matches!(app.state.input_mode, InputMode::Command) {
        format!(" Command (/ for commands, Tab to autocomplete){suffix}")
    } else {
        format!(" Message (Enter to send, Ctrl+V image){suffix}")
    };
    let border_color = if app.state.processing {
        Color::Yellow
    } else if matches!(app.state.input_mode, InputMode::Command) {
        Color::Magenta
    } else {
        palette.border
    };
    let input = Paragraph::new(app.state.input.as_str())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(border_color))
                .title(title),
        )
        .wrap(Wrap { trim: false });
    f.render_widget(input, area);
    place_cursor(f, app, area);
}
