//! Visibility and scroll state for the help overlay.
use crate::HelpScrollState;

/// Interactive state retained while opening and closing help.
///
/// The frontend decides when to reset scrolling. Toggling visibility alone
/// preserves the existing offset.
///
/// # Examples
/// ```
/// let mut help = codetether_help_view::HelpState::default();
/// assert!(!help.visible);
/// help.visible = true;
/// help.scroll.scroll_down(2, 10);
/// assert_eq!(help.scroll.offset, 2);
/// ```
#[derive(Default)]
pub struct HelpState {
    /// Whether the help overlay is open.
    pub visible: bool,
    /// Scroll position retained by the help overlay.
    pub scroll: HelpScrollState,
}
