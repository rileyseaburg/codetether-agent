//! Focus-aware rounded pane borders.
//!
//! Provides a consistent border treatment across TUI panes: the focused
//! pane gets a bright, bold cyan rounded border; unfocused panes get a dim
//! dark-gray one. Pure styling helpers — no application state, no compute.

use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

fn border_style(focused: bool) -> Style {
    if focused {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
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
