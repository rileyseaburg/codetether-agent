//! Per-view accent color.
//!
//! Maps each [`ViewMode`] to a signature accent color so the active screen
//! can tint its header and borders, giving an at-a-glance sense of location.
//! Pure lookup — no state, no compute.

use ratatui::style::Color;

use crate::tui::models::ViewMode;

/// Return the accent [`Color`] for a given [`ViewMode`].
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::mode_accent::mode_accent;
/// use codetether_agent::tui::models::ViewMode;
/// use ratatui::style::Color;
/// assert_eq!(mode_accent(&ViewMode::Chat), Color::Cyan);
/// ```
pub fn mode_accent(mode: &ViewMode) -> Color {
    match mode {
        ViewMode::Chat => Color::Cyan,
        ViewMode::Sessions => Color::Green,
        ViewMode::Swarm => Color::Magenta,
        ViewMode::Ralph => Color::Blue,
        ViewMode::Bus => Color::LightCyan,
        ViewMode::Model => Color::LightBlue,
        ViewMode::Settings => Color::Gray,
        ViewMode::Lsp => Color::LightGreen,
        ViewMode::Rlm => Color::LightMagenta,
        ViewMode::Latency => Color::LightRed,
        ViewMode::Protocol => Color::LightYellow,
        ViewMode::FilePicker => Color::Green,
        ViewMode::Inspector => Color::LightCyan,
        ViewMode::Audit => Color::Yellow,
        ViewMode::Git => Color::LightRed,
        ViewMode::AuditLoop => Color::Yellow,
    }
}
