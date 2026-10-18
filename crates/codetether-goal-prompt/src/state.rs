//! Owned goal list, current text, and response sender.
use tokio::sync::oneshot;

/// State for the active goal-setup modal.
///
/// While `Some(GoalPromptState)` is stored on the app, the event loop routes
/// all keystrokes here and the overlay renders the card. Submitting (or
/// skipping) sends the collected goals through `responder` and clears state.
///
/// # Examples
/// ```
/// let (tx, _rx) = tokio::sync::oneshot::channel();
/// let state = codetether_goal_prompt::GoalPromptState::new("Goals", "Add a goal", tx);
/// assert!(state.goals.is_empty());
/// ```
pub struct GoalPromptState {
    /// Big friendly heading, e.g. "Let's set up your goals".
    pub title: String,
    /// Plain-English explanation shown under the title.
    pub body: String,
    /// Goals the user has added so far.
    pub goals: Vec<String>,
    /// The line currently being typed.
    pub current: String,
    /// Channel used to hand the result back to the waiting task.
    pub responder: Option<oneshot::Sender<Vec<String>>>,
}
