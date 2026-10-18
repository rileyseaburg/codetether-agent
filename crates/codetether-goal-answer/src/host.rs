//! Native journal, UI and answer-generation boundaries.
use codetether_goal_core::{AnswerReviewAction, TaskState};
use std::future::Future;
/// Durable human-only answer-review operations.
/// # Implementors
/// Bind these operations to the existing session journal and user controls.
pub trait Journal {
    /// Read current goal and answer-review state.
    /// # Errors
    /// Returns storage errors.
    fn read(id: &str) -> anyhow::Result<TaskState>;
    /// Check whether the session has a held answer review.
    fn held(id: &str) -> bool;
    /// Begin a held question if the current goal permits it.
    /// # Errors
    /// Returns journal update errors.
    fn begin(id: &str, question: &str) -> impl Future<Output = anyhow::Result<bool>>;
    /// Record a human answer-review decision.
    /// # Errors
    /// Returns journal update errors.
    fn record(
        id: &str,
        goal: &str,
        review: &str,
        action: AnswerReviewAction,
    ) -> impl Future<Output = anyhow::Result<()>>;
}
/// Native composer/display updates for held-question input.
/// # Implementors
/// Preserve the frontend's existing history, cursor and status conventions.
pub trait InputView {
    /// Current composer text.
    fn input(&self) -> &str;
    /// Replace composer text and move its cursor to the end.
    fn rewrite(&mut self, text: String);
    /// Replace status text.
    fn status(&mut self, text: String);
    /// Append submitted input to history.
    fn history(&mut self, text: String);
    /// Clear the composer.
    fn clear(&mut self);
    /// Reset the answer-review choice to No.
    fn answer_no(&mut self);
    /// Request display refresh.
    fn redraw(&mut self);
}
/// Session identity, journal and cancellation for held-question input.
/// # Implementors
/// Do not borrow the in-flight session to obtain its identity.
pub trait InputActions {
    /// Durable journal binding.
    type Journal: Journal;
    /// Current session identity from its read-only view.
    fn session_id(&self) -> String;
    /// Request cancellation of the current execution.
    fn cancel(&mut self);
    /// Schedule continuation once ownership returns.
    fn restart(&mut self, id: &str);
}
/// Complete native question-input interface.
/// # Implementors
/// All providers of input state and actions implement this trait.
pub trait Input: InputView + InputActions {}
impl<T: InputView + InputActions> Input for T {}
/// Native session availability and answer generation.
/// # Implementors
/// Answer using the existing read-only ask path once the session is available.
pub trait Delivery {
    /// Durable journal binding.
    type Journal: Journal;
    /// Whether a turn is still processing.
    fn processing(&self) -> bool;
    /// Identity of the available session, or none while ownership is elsewhere.
    fn available_session(&self) -> Option<String>;
    /// Generate and present an answer through the existing ask implementation.
    fn ask(&mut self, question: &str) -> impl Future<Output = bool>;
    /// Replace status text.
    fn status(&mut self, text: String);
    /// Reset the answer-review choice to No.
    fn answer_no(&mut self);
    /// Scroll to the latest message.
    fn scroll(&mut self);
    /// Request display refresh.
    fn redraw(&mut self);
}
