//! Context-health command parsing and report presentation.
use crate::ContextHealthState;
/// Read-only context state and application presentation operations.
pub trait View {
    /// Borrow recent context events and current token estimates.
    fn context(&self) -> (&ContextHealthState, Option<usize>, Option<usize>);
    /// Append the report to chat.
    fn push_system_message(&mut self, text: String);
    /// Scroll to the report.
    fn scroll_to_bottom(&mut self);
    /// Replace the command status line.
    fn set_status(&mut self, text: String);
}
/// Handle the context status command or report its existing usage text.
/// # Arguments
/// * `view` — Context projection and chat presentation.
/// * `rest` — Text after the command name.
/// # Returns
/// Updates chat for an empty/status argument and status only for other arguments.
/// # Examples
/// ```no_run
/// # fn status(view: &mut impl codetether_context_health::command::View) {
/// codetether_context_health::command::execute(view, "status");
/// # }
/// ```
pub fn execute(view: &mut impl View, rest: &str) {
    match rest.trim() {
        "" | "status" => {
            let (state, used, budget) = view.context();
            let body = crate::render(state, used, budget);
            view.push_system_message(body);
            view.scroll_to_bottom();
            view.set_status("Context status".to_string());
        }
        _ => view.set_status("Usage: /context [status]".to_string()),
    }
}
