//! Scroll offset for the help overlay.

/// Mutable scroll state for the help view, stored externally.
/// # Examples
/// ```
/// let mut state = codetether_help_view::HelpScrollState::default();
/// state.scroll_down(2, 10);
/// assert_eq!(state.offset, 2);
/// ```
pub struct HelpScrollState {
    /// First visible help row.
    pub offset: usize,
}

impl Default for HelpScrollState {
    fn default() -> Self {
        Self { offset: 0 }
    }
}

impl HelpScrollState {
    /// Move upward, saturating at the first row.
    pub fn scroll_up(&mut self, n: usize) {
        self.offset = self.offset.saturating_sub(n);
    }

    /// Move downward, clamping at the supplied maximum.
    pub fn scroll_down(&mut self, n: usize, max: usize) {
        self.offset = (self.offset + n).min(max);
    }
}
