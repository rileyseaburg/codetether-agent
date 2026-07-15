//! Age-based color for session list rows.
//!
//! Recent sessions glow bright cyan; older ones fade toward dim gray,
//! giving the session picker an instant visual age gradient.

use chrono::{DateTime, Utc};
use ratatui::style::Color;

use crate::tui::ui::gradient::lerp_rgb;

const FRESH: (u8, u8, u8) = (0, 229, 255);
const STALE: (u8, u8, u8) = (80, 80, 100);
const MAX_HOURS: f32 = 168.0; // 1 week

/// Return a color representing how recently `updated_at` occurred.
///
/// # Examples
///
/// ```rust
/// use chrono::Utc;
/// use codetether_agent::tui::ui::sessions_age_color::age_color;
/// let _c = age_color(Utc::now(), false);
/// ```
pub fn age_color(updated_at: DateTime<Utc>, rgb: bool) -> Color {
    let hours = (Utc::now() - updated_at).num_seconds().max(0) as f32 / 3_600.0;
    if rgb {
        let t = (hours / MAX_HOURS).clamp(0.0, 1.0);
        lerp_rgb(FRESH, STALE, t)
    } else {
        match hours as u64 {
            0 => Color::Cyan,
            1..=5 => Color::Green,
            6..=23 => Color::Yellow,
            _ => Color::DarkGray,
        }
    }
}
