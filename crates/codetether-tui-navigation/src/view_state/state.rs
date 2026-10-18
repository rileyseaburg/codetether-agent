//! Active panel and saved scroll positions owned by navigation.
use super::SavedScroll;
use crate::ViewTransition;
use codetether_tui_models::ViewMode;

/// Current panel selection and the scroll snapshot retained across view changes.
///
/// # Examples
/// ```
/// let state = codetether_tui_navigation::view_state::ViewState::default();
/// assert_eq!(state.mode, codetether_tui_models::ViewMode::Chat);
/// ```
pub struct ViewState {
    /// Currently selected panel.
    pub mode: ViewMode,
    /// Chat and tool-preview scrolling retained when leaving chat.
    pub saved_scroll: SavedScroll,
}

impl Default for ViewState {
    fn default() -> Self {
        Self {
            mode: ViewMode::Chat,
            saved_scroll: SavedScroll::default(),
        }
    }
}

impl ViewState {
    /// Borrow the active view and current viewport fields for a transition.
    /// # Arguments
    /// Receives the current chat scroll, follow flag and tool-preview scroll.
    /// # Returns
    /// A transition adapter borrowing this state and the supplied viewport fields.
    /// # Examples
    /// ```
    /// let mut state = codetether_tui_navigation::view_state::ViewState::default();
    /// let (mut chat, mut follow, mut tool) = (0, true, 0);
    /// state.transition(&mut chat, &mut follow, &mut tool)
    ///     .set_view_mode(codetether_tui_models::ViewMode::Bus);
    /// assert_eq!(state.mode, codetether_tui_models::ViewMode::Bus);
    /// ```
    pub fn transition<'a>(
        &'a mut self,
        chat_scroll: &'a mut usize,
        chat_auto_follow: &'a mut bool,
        tool_scroll: &'a mut usize,
    ) -> ViewTransition<'a> {
        ViewTransition {
            mode: &mut self.mode,
            saved: &mut self.saved_scroll,
            chat_scroll,
            chat_auto_follow,
            tool_scroll,
        }
    }
}
