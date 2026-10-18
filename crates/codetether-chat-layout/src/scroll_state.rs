//! Chat scroll position and follow-latest state.
//! The shared state owns viewport positions without application events.

/// Mutable chat viewport position and its most recent content bound.
///
/// # Examples
/// ```
/// let state = codetether_chat_layout::ChatScrollState::default();
/// assert!(state.chat_auto_follow);
/// ```
#[derive(Debug, Clone)]
pub struct ChatScrollState {
    /// Current position, or SCROLL_BOTTOM to follow the latest output.
    pub chat_scroll: usize,
    /// Last measured maximum scroll position.
    pub chat_last_max_scroll: usize,
    /// Whether new messages should keep following the bottom.
    pub chat_auto_follow: bool,
}
impl Default for ChatScrollState {
    fn default() -> Self {
        Self {
            chat_scroll: 0,
            chat_last_max_scroll: 0,
            chat_auto_follow: true,
        }
    }
}
