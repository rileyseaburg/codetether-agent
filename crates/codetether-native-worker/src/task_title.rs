//! Preserve resumed session titles while avoiding worker title-generation calls.
/// Session title access used by worker task preparation.
/// # Implementors
/// Applications forward to their existing title storage and setter.
pub trait Title {
    /// Current title, if the session already has one.
    fn task_title(&self) -> Option<&str>;
    /// Persist the supplied title in the session's normal in-memory state.
    fn set_task_title(&mut self, title: String);
}
/// Preseed a task's known title only when the session has no existing title.
/// # Arguments
/// * `session` - Session whose title is inspected and optionally set.
/// * `title` - Human-readable worker task title.
/// # Returns
/// Completes without changing a resumed title.
/// # Examples
/// ```
/// use codetether_native_worker::task_title::{Title, preseed_task_title};
/// struct Session(Option<String>);
/// impl Title for Session {
///     fn task_title(&self) -> Option<&str> { self.0.as_deref() }
///     fn set_task_title(&mut self, title: String) { self.0 = Some(title); }
/// }
/// let mut session = Session(None);
/// preseed_task_title(&mut session, "Task");
/// preseed_task_title(&mut session, "Replacement");
/// assert_eq!(session.task_title(), Some("Task"));
/// ```
pub fn preseed_task_title(session: &mut impl Title, title: &str) {
    if session.task_title().is_none() {
        session.set_task_title(title.to_string());
    }
}
