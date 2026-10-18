//! Borrowed application values required to draw the picker.
use codetether_session_metadata::SessionSummary;
/// Session selection and presentation inputs, without application ownership.
///
/// # Examples
/// ```
/// use codetether_session_picker::PickerView;
/// let view = PickerView { sessions: &[], filter: "", selected: 0,
///     active_session: None, status: "Ready" };
/// assert!(view.sessions.is_empty());
/// ```
pub struct PickerView<'a> {
    /// Available session summaries in listing order.
    pub sessions: &'a [SessionSummary],
    /// Fuzzy query across session identifiers and titles.
    pub filter: &'a str,
    /// Selection index within the filtered collection.
    pub selected: usize,
    /// Identifier of the currently loaded session.
    pub active_session: Option<&'a str>,
    /// Application status displayed in the heading.
    pub status: &'a str,
}
