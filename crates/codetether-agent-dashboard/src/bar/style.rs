//! Identity-color style for agent tabs.

use ratatui::style::{Color, Modifier, Style};

use codetether_theme::agent_color::agent_color;
use codetether_theme::gradient::rgb_supported;

/// Build the tab [`Style`] for `name` — selected or dim.
///
/// # Examples
///
/// ```rust
/// use codetether_agent_dashboard::bar::style::tab_style;
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
