//! Stable per-agent colors derived from identity names.
//! [agent_color] hashes a name to a hue, with a named-color fallback.
use ratatui::style::Color;
mod hue;
use hue::{hue_to_rgb, name_hue};

const NAMED_COLORS: [Color; 8] = [
    Color::Red,
    Color::Yellow,
    Color::Green,
    Color::Cyan,
    Color::Blue,
    Color::Magenta,
    Color::LightRed,
    Color::LightGreen,
];

/// Return a stable neon-saturated [`Color`] for the given agent name.
///
/// On non-truecolor terminals returns a cycling named color instead.
///
/// # Examples
///
/// ```rust
/// use codetether_theme::agent_color::agent_color;
/// let c1 = agent_color("researcher", true);
/// let c2 = agent_color("researcher", true);
/// assert_eq!(c1, c2); // deterministic
/// ```
pub fn agent_color(name: &str, rgb: bool) -> Color {
    let hue = name_hue(name);
    if rgb {
        hue_to_rgb(hue)
    } else {
        NAMED_COLORS[((hue as usize) / 45) % NAMED_COLORS.len()]
    }
}
