//! Focus-aware rounded pane borders.
//!
//! Focused panes get a neon animated border whose color cycles with the
//! global spinner hue (truecolor) or stays bold cyan (8-color). Unfocused
//! panes are dim dark-gray. Pure styling helpers — no application state.

use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

use crate::tui::ui::chat_view::spinner::spinner_color;
use crate::tui::ui::gradient::rgb_supported;

fn border_style(focused: bool) -> Style {
    if focused {
        let color = if rgb_supported() {
            spinner_color()
        } else {
            Color::Cyan
        };
        Style::default().fg(color).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

/// A rounded-border [`Block`] styled by focus state.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::border_style::pane_border;
/// let _focused = pane_border(true);
/// let _dim = pane_border(false);
/// ```
pub fn pane_border(focused: bool) -> Block<'static> {
    Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(border_style(focused))
}

/// A rounded-border [`Block`] with a title, styled by focus state.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::border_style::pane_border_titled;
/// let _b = pane_border_titled(true, "Chat");
/// ```
pub fn pane_border_titled(focused: bool, title: &str) -> Block<'static> {
    pane_border(focused).title(title.to_string())
}
