//! Neon highlight style for the autocomplete suggestions list.

use ratatui::style::{Color, Style};

use codetether_tui_style::gradient::rgb_supported;
use codetether_tui_style::spinner::spinner_color;

/// Build the selected autocomplete row style.
///
/// # Arguments
/// No arguments.
///
/// # Returns
/// A neon or named-color fallback highlight style.
///
/// # Examples
/// ```
/// let _style = codetether_composer_view::highlight_style();
/// ```
pub fn highlight_style() -> Style {
    if rgb_supported() {
        Style::default().bg(spinner_color()).fg(Color::Black).bold()
    } else {
        Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold()
    }
}
