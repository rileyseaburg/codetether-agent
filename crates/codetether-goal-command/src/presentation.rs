//! Frontend presentation for human goal-command results.
use crate::Host;
/// Presentation effects following an explicitly requested goal command.
/// # Implementors
/// Frontends append system notices and replace their status line.
pub trait View {
    /// Append the successful result as a system notice.
    fn push_system_message(&mut self, message: String);
    /// Replace the frontend status line.
    fn set_status(&mut self, status: String);
}
/// Execute a human goal command and present its result.
/// # Arguments
/// * `view` — Frontend result presentation.
/// * `session_id` — Session whose authorized goal command runs.
/// * `raw` — Command arguments after the goal prefix.
/// # Returns
/// True on successful command execution; false after displaying an error.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_goal_command::Host>(view: &mut impl codetether_goal_command::presentation::View) {
/// let _ = codetether_goal_command::presentation::handle::<H>(view, "session", "status").await;
/// # }
/// ```
pub async fn handle<H: Host>(view: &mut impl View, session_id: &str, raw: &str) -> bool {
    let result = crate::run::<H>(session_id, raw).await;
    match result {
        Ok(message) => {
            view.push_system_message(message.clone());
            view.set_status(message.lines().next().unwrap_or("Goal updated").to_string());
            true
        }
        Err(error) => {
            view.set_status(format!("/goal: {error}"));
            false
        }
    }
}
