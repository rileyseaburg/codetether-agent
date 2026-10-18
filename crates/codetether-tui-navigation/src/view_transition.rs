//! View changes coordinated with saved chat and tool-preview scrolling.
use crate::view_state::{SavedScroll, set_view_mode};
use codetether_tui_models::ViewMode;

/// Borrowed view and scroll fields used when opening or closing panels.
///
/// # Examples
/// ```
/// use codetether_tui_navigation::{ViewTransition, view_state::SavedScroll};
/// let mut mode = codetether_tui_models::ViewMode::Chat;
/// let mut saved = SavedScroll::default();
/// let (mut chat, mut follow, mut tool) = (12, false, 4);
/// let mut view = ViewTransition {
///     mode: &mut mode, saved: &mut saved, chat_scroll: &mut chat,
///     chat_auto_follow: &mut follow, tool_scroll: &mut tool,
/// };
/// view.set_view_mode(codetether_tui_models::ViewMode::Bus);
/// assert_eq!(*view.mode, codetether_tui_models::ViewMode::Bus);
/// ```
pub struct ViewTransition<'a> {
    /// Current panel selection.
    pub mode: &'a mut ViewMode,
    /// Saved scroll positions for restoring chat.
    pub saved: &'a mut SavedScroll,
    /// Current chat scroll position.
    pub chat_scroll: &'a mut usize,
    /// Whether chat follows new output automatically.
    pub chat_auto_follow: &'a mut bool,
    /// Current tool-preview scroll position.
    pub tool_scroll: &'a mut usize,
}

impl ViewTransition<'_> {
    /// Select a panel, saving scroll positions when leaving chat.
    pub fn set_view_mode(&mut self, next: ViewMode) {
        set_view_mode(
            self.mode,
            next,
            self.saved,
            *self.chat_scroll,
            *self.chat_auto_follow,
            *self.tool_scroll,
        );
    }

    /// Save current chat and tool-preview scrolling explicitly.
    pub fn save_scroll_state(&mut self) {
        self.saved
            .save(*self.chat_scroll, *self.chat_auto_follow, *self.tool_scroll);
    }

    /// Restore the saved chat and tool-preview scroll positions.
    pub fn restore_scroll_state(&mut self) {
        self.saved
            .restore(self.chat_scroll, self.chat_auto_follow, self.tool_scroll);
    }
}
