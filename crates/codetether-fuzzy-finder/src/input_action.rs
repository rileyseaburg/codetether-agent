//! Host actions emitted by fuzzy finder input.
//! Match FinderAction after handling a key to close or open an editor.

/// Action requested by an open fuzzy finder.
///
/// # Examples
/// ```
/// use codetether_fuzzy_finder::FinderAction;
/// assert!(matches!(FinderAction::Continue, FinderAction::Continue));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FinderAction {
    /// Continue showing the finder after updating or ignoring input.
    Continue,
    /// Close the finder without opening a file.
    Cancel,
    /// Open the currently selected file.
    Confirm,
}
