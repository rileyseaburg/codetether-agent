//! Per-view accent color.
//!
//! Maps each [`ViewMode`] to a signature accent color so the active screen
//! can tint its header and borders, giving an at-a-glance sense of location.
//! On truecolor terminals each view gets a rich, distinct RGB accent; on
//! 8/256-color terminals the classic named colors are used.

use ratatui::style::Color;

use crate::gradient::rgb_supported;
use codetether_tui_models::ViewMode;

/// Return the accent [`Color`] for a given [`ViewMode`].
///
/// # Examples
///
/// ```rust
/// use codetether_tui_style::mode_accent::mode_accent;
/// use codetether_tui_models::ViewMode;
/// let _c = mode_accent(&ViewMode::Chat);
/// ```
///
/// # Arguments
/// * `mode` - Active application view.
///
/// # Returns
/// The rich or fallback accent color.
pub fn mode_accent(mode: &ViewMode) -> Color {
    if rgb_supported() {
        rich_accent(mode)
    } else {
        named_accent(mode)
    }
}

#[path = "accent_named.rs"]
mod named;
#[path = "accent_rich.rs"]
mod rich;
use named::named_accent;
use rich::rich_accent;
