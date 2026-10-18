//! Focus-aware rounded pane borders.
//!
//! Focused panes get a neon animated border whose color cycles with the
//! global spinner hue (truecolor) or stays bold cyan (8-color). Unfocused
//! panes are dim dark-gray. Pure styling helpers — no application state.

use ratatui::{
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

use crate::gradient::rgb_supported;
use crate::spinner::spinner_color;

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
/// use codetether_tui_style::border_style::pane_border;
/// let _focused = pane_border(true);
/// let _dim = pane_border(false);
/// ```
///
/// # Arguments
/// * `focused` - Whether this pane has focus.
///
/// # Returns
/// A rounded block with active or dim border styling.
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
/// use codetether_tui_style::border_style::pane_border_titled;
/// let _b = pane_border_titled(true, "Chat");
/// ```
///
/// # Arguments
/// * `focused` - Whether this pane has focus.
/// * `title` - Owned copy of the pane title.
///
/// # Returns
/// A rounded, titled block.
pub fn pane_border_titled(focused: bool, title: &str) -> Block<'static> {
    pane_border(focused).title(title.to_string())
}
