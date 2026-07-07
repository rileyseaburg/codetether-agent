//! Identity-color style for agent tabs.

use ratatui::style::{Color, Modifier, Style};

use crate::tui::ui::agent_color::agent_color;
use crate::tui::ui::gradient::rgb_supported;

/// Build the tab [`Style`] for `name` — selected or dim.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::chat_view::agent_tab_style::tab_style;
/// let _s = tab_style("researcher", true);
/// ```
pub fn tab_style(name: &str, selected: bool) -> Style {
    let color = agent_color(name, rgb_supported());
    if selected {
        Style::default()
            .fg(Color::Black)
            .bg(color)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(color).add_modifier(Modifier::DIM)
    }
}
