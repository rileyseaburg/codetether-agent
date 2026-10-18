//! Chat scrolling with frontend-supplied older-history requests.
use crate::ChatScrollState;

/// Borrowed viewport and page size for manual chat navigation.
///
/// The callback receives the unsatisfied upward movement at the start of the
/// visible buffer. Storage and background loading remain with the frontend.
///
/// # Examples
/// ```
/// let mut viewport = codetether_chat_layout::ChatScrollState::default();
/// let mut requested = None;
/// let mut navigation = codetether_chat_layout::Navigation {
///     viewport: &mut viewport, page_step: 9,
///     load_older: |amount| requested = Some(amount),
/// };
/// navigation.page_up();
/// drop(navigation);
/// assert_eq!(requested, Some(9));
/// ```
pub struct Navigation<'a, F> {
    /// Viewport updated by manual navigation.
    pub viewport: &'a mut ChatScrollState,
    /// Number of rows moved by page navigation.
    pub page_step: usize,
    /// Request older history when upward movement reaches the buffer start.
    pub load_older: F,
}

impl<F: FnMut(usize)> Navigation<'_, F> {
    /// Move upward and request older history for any unmet movement.
    pub fn scroll_up(&mut self, amount: usize) {
        if let Some(deficit) = self.viewport.scroll_up(amount) {
            (self.load_older)(deficit);
        }
    }

    /// Move upward by one page using the same history-request path.
    pub fn page_up(&mut self) {
        self.scroll_up(self.page_step);
    }

    /// Move downward by one page and restore following at the end.
    pub fn page_down(&mut self) {
        self.viewport.scroll_down(self.page_step);
    }
}
