//! Bounded prior inputs and the active recall position.
/// Command-history storage owned by the composer.
///
/// # Examples
/// ```
/// let history = codetether_composer_view::CommandHistory::default();
/// assert!(history.command_history.is_empty());
/// ```
#[derive(Default)]
pub struct CommandHistory {
    /// Prior submitted inputs in chronological order.
    pub command_history: Vec<String>,
    /// Active recall index, or none when editing new input.
    pub history_index: Option<usize>,
}
/// The next history entry or the end of recalled history.
///
/// # Examples
/// ```
/// use codetether_composer_view::HistoryRecall;
/// assert!(matches!(HistoryRecall::End, HistoryRecall::End));
/// ```
pub enum HistoryRecall {
    /// Replace input with this recalled entry.
    Entry(String),
    /// Clear input and return to editing a new command.
    End,
}
