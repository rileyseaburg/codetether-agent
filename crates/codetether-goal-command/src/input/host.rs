//! Frontend state and command operations for human goal input.
use codetether_goal_core::TaskState;
use std::future::Future;
/// Input and display state used by command interception.
/// # Implementors
/// Implement with direct UI updates, preserving history and redraw semantics.
pub trait View {
    /// Current input text.
    fn input(&self) -> &str;
    /// Replace command status.
    fn status(&mut self, text: String);
    /// Record submitted input in history.
    fn history(&mut self, text: String);
    /// Clear the composer.
    fn clear(&mut self);
    /// Request a redraw.
    fn redraw(&mut self);
    /// Append a system message.
    fn system(&mut self, text: String);
    /// Scroll to the latest message.
    fn scroll(&mut self);
}
/// Session-bound command actions independent of in-flight session ownership.
/// # Implementors
/// Read identity from the session view without borrowing the active session.
pub trait Actions {
    /// Current session identity.
    fn session_id(&self) -> String;
    /// Execute the existing human goal command and return whether it changed state.
    fn goal(&mut self, id: &str, text: &str) -> impl Future<Output = bool>;
    /// Read task state using the existing session storage path.
    /// # Errors
    /// Returns session journal errors.
    fn tasks(&self, id: &str) -> anyhow::Result<TaskState>;
    /// Open the human goal editor.
    fn edit(&mut self, id: &str);
    /// Schedule restart after returning session ownership.
    fn restart(&mut self, id: &str);
}
/// Complete frontend for goal/task command interception.
/// # Implementors
/// Every implementation of both focused interfaces implements this trait.
pub trait Host: View + Actions {}
impl<T: View + Actions> Host for T {}
