//! Neon highlight style for the autocomplete suggestions list.

use ratatui::style::{Color, Style};

use crate::tui::ui::chat_view::spinner::spinner_color;
use crate::tui::ui::gradient::rgb_supported;

/// Animated neon highlight for the selected suggestion row.
pub fn highlight_style() -> Style {
    if rgb_supported() {
        Style::default().bg(spinner_color()).fg(Color::Black).bold()
    } else {
        Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold()
    }
}
