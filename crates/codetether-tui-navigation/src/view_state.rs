//! Saved chat scroll state and transitions away from the chat view.
use codetether_tui_models::ViewMode;
mod state;
pub use state::ViewState;
/// Scroll positions retained while another view is active.
///
/// # Examples
/// ```
/// let mut saved = codetether_tui_navigation::view_state::SavedScroll::default();
/// saved.save(12, false, 4);
/// let (mut chat, mut follow, mut tool) = (0, true, 0);
/// saved.restore(&mut chat, &mut follow, &mut tool);
/// assert_eq!((chat, follow, tool), (12, false, 4));
/// ```
#[derive(Debug, Clone)]
pub struct SavedScroll {
    chat_scroll: usize,
    chat_auto_follow: bool,
    tool_preview_scroll: usize,
}
impl Default for SavedScroll {
    fn default() -> Self {
        Self {
            chat_scroll: 0,
            chat_auto_follow: true,
            tool_preview_scroll: 0,
        }
    }
}
impl SavedScroll {
    /// Capture the current chat and tool scroll positions.
    /// # Arguments
    /// Receives chat position, automatic following, and tool-preview position.
    /// # Returns
    /// Replaces the previously saved values.
    pub fn save(&mut self, chat_scroll: usize, chat_auto_follow: bool, tool_preview_scroll: usize) {
        self.chat_scroll = chat_scroll;
        self.chat_auto_follow = chat_auto_follow;
        self.tool_preview_scroll = tool_preview_scroll;
    }
    /// Restore the saved values into the current viewports.
    /// # Arguments
    /// Receives mutable chat position, automatic following and tool-preview position.
    /// # Returns
    /// Copies the saved values into the supplied fields.
    pub fn restore(
        &self,
        chat_scroll: &mut usize,
        chat_auto_follow: &mut bool,
        tool_preview_scroll: &mut usize,
    ) {
        *chat_scroll = self.chat_scroll;
        *chat_auto_follow = self.chat_auto_follow;
        *tool_preview_scroll = self.tool_preview_scroll;
    }
}
/// Change views, capturing scroll positions only when leaving chat.
/// # Arguments
/// Receives current/next view, saved state, and the three current viewport values.
/// # Returns
/// Updates the current view after any required snapshot.
/// # Examples
/// ```
/// use codetether_tui_navigation::view_state::{SavedScroll, set_view_mode};
/// let mut view = codetether_tui_models::ViewMode::Chat;
/// let mut saved = SavedScroll::default();
/// set_view_mode(&mut view, codetether_tui_models::ViewMode::Bus, &mut saved, 12, false, 4);
/// assert_eq!(view, codetether_tui_models::ViewMode::Bus);
/// ```
pub fn set_view_mode(
    current: &mut ViewMode,
    next: ViewMode,
    saved: &mut SavedScroll,
    chat_scroll: usize,
    chat_auto_follow: bool,
    tool_preview_scroll: usize,
) {
    if *current == ViewMode::Chat && next != ViewMode::Chat {
        saved.save(chat_scroll, chat_auto_follow, tool_preview_scroll);
    }
    *current = next;
}
