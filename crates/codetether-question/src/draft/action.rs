//! Editing actions accepted by a question draft.
/// Frontend-independent editing actions. Dismissal is handled by the runtime.
///
/// # Examples
/// ```
/// use codetether_question::Action;
/// assert!(matches!(Action::Next, Action::Next));
/// ```
#[derive(Clone, Debug)]
pub enum Action {
    /// Move to the next option, wrapping through the custom-answer row.
    Next,
    /// Move to the previous option.
    Previous,
    /// Toggle the highlighted choice for a multiple-selection question.
    Toggle,
    /// Add text to the custom answer and focus it.
    Text(String),
    /// Remove the last Unicode scalar from the custom answer.
    Backspace,
    /// Go back to the previous question without losing answers.
    Back,
    /// Commit the current question; finish after the last question.
    Submit,
}
