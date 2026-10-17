//! Parsed history browsing action.

/// Parsed form of the JSON `args` payload.
///
/// # Examples
/// ```rust
/// use codetether_context_browse::ContextBrowseAction;
/// let action=ContextBrowseAction::ShowTurn{turn:3};
/// assert!(matches!(action,ContextBrowseAction::ShowTurn{turn:3}));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextBrowseAction {
    /// Default action — enumerate every turn as a virtual path.
    ListTurns,
    /// Return the text body of the turn at `turn`.
    ShowTurn { turn: usize },
}
