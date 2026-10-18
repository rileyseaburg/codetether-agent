//! Owned session listing, filter, and cursor state.
use codetether_session_metadata::SessionSummary;
/// State shared by session-picker navigation and rendering.
///
/// # Examples
/// ```
/// use codetether_session_picker::SessionPickerState;
/// let state = SessionPickerState::default();
/// assert!(state.filtered_sessions().is_empty());
/// ```
#[derive(Default)]
pub struct SessionPickerState {
    /// Cached summaries in listing order.
    pub sessions: Vec<SessionSummary>,
    /// Cursor index into the filtered collection.
    pub selected_session: usize,
    /// Fuzzy query across session identifiers and titles.
    pub session_filter: String,
}
