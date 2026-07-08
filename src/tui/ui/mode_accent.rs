//! Per-view accent color.
//!
//! Maps each [`ViewMode`] to a signature accent color so the active screen
//! can tint its header and borders, giving an at-a-glance sense of location.
//! On truecolor terminals each view gets a rich, distinct RGB accent; on
//! 8/256-color terminals the classic named colors are used.

use ratatui::style::Color;

use crate::tui::models::ViewMode;
use crate::tui::ui::gradient::rgb_supported;

/// Return the accent [`Color`] for a given [`ViewMode`].
///
/// # Examples
///
/// ```rust
/// use codetether_agent::tui::ui::mode_accent::mode_accent;
/// use codetether_agent::tui::models::ViewMode;
/// let _c = mode_accent(&ViewMode::Chat);
/// ```
pub fn mode_accent(mode: &ViewMode) -> Color {
    if rgb_supported() {
        rich_accent(mode)
    } else {
        named_accent(mode)
    }
}

fn rich_accent(mode: &ViewMode) -> Color {
    match mode {
        ViewMode::Chat => Color::Rgb(0, 229, 255),
        ViewMode::Sessions => Color::Rgb(0, 255, 128),
        ViewMode::Swarm => Color::Rgb(255, 64, 216),
        ViewMode::Ralph => Color::Rgb(100, 80, 255),
        ViewMode::Bus => Color::Rgb(0, 200, 255),
        ViewMode::Model => Color::Rgb(80, 160, 255),
        ViewMode::Settings => Color::Rgb(160, 160, 160),
        ViewMode::Lsp => Color::Rgb(80, 255, 160),
        ViewMode::Rlm => Color::Rgb(220, 80, 255),
        ViewMode::Latency => Color::Rgb(255, 100, 80),
        ViewMode::Transport => Color::Rgb(80, 180, 255),
        ViewMode::Protocol => Color::Rgb(255, 220, 60),
        ViewMode::FilePicker => Color::Rgb(60, 220, 120),
        ViewMode::Inspector => Color::Rgb(60, 220, 220),
        ViewMode::Audit => Color::Rgb(255, 180, 0),
        ViewMode::Git => Color::Rgb(255, 80, 80),
        ViewMode::AuditLoop => Color::Rgb(255, 160, 0),
        ViewMode::Editor => Color::Rgb(120, 120, 255),
    }
}

fn named_accent(mode: &ViewMode) -> Color {
    match mode {
        ViewMode::Chat => Color::Cyan,
        ViewMode::Sessions | ViewMode::FilePicker => Color::Green,
        ViewMode::Swarm => Color::Magenta,
        ViewMode::Ralph => Color::Blue,
        ViewMode::Bus | ViewMode::Inspector => Color::LightCyan,
        ViewMode::Model | ViewMode::Transport | ViewMode::Editor => Color::LightBlue,
        ViewMode::Settings => Color::Gray,
        ViewMode::Lsp => Color::LightGreen,
        ViewMode::Rlm => Color::LightMagenta,
        ViewMode::Latency | ViewMode::Git => Color::LightRed,
        ViewMode::Protocol => Color::LightYellow,
        ViewMode::Audit | ViewMode::AuditLoop => Color::Yellow,
    }
}
