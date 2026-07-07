//! Stable per-agent color derived from the agent name.
//!
//! Hashes the name to a hue in `[0°, 360°)`, then converts to a
//! saturated RGB color. The same name always produces the same color
//! across frames and sessions, so multi-agent sessions read like a
//! channel list: each participant has a persistent visual identity.

use ratatui::style::Color;

/// Return a stable neon-saturated [`Color`] for the given agent name.
///
/// On non-truecolor terminals returns a cycling named color instead.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::agent_color::agent_color;
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

fn name_hue(name: &str) -> f32 {
    // FNV-1a fold to a float hue.
    let mut h: u32 = 2_166_136_261;
    for b in name.bytes() {
        h ^= u32::from(b);
        h = h.wrapping_mul(16_777_619);
    }
    (h % 360) as f32
}

fn hue_to_rgb(hue: f32) -> Color {
    let h = (hue % 360.0) / 60.0;
    let x = 1.0 - (h % 2.0 - 1.0).abs();
    let (r, g, b) = match h as u32 {
        0 => (1.0_f32, x, 0.0_f32),
        1 => (x, 1.0, 0.0),
        2 => (0.0, 1.0, x),
        3 => (0.0, x, 1.0),
        4 => (x, 0.0, 1.0),
        _ => (1.0, 0.0, x),
    };
    let s = |v: f32| (v * 255.0).round() as u8;
    Color::Rgb(s(r), s(g), s(b))
}
