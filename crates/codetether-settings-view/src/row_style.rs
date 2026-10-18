//! Settings row style.
use ratatui::style::{Color, Style};

pub(super) fn row_style(selected: bool) -> Style {
    if selected {
        Style::default().fg(Color::Cyan).bold()
    } else {
        Style::default()
    }
}
