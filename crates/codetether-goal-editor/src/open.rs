//! Open or discard a virtual goal draft.
use crate::{Draft, Host, Journal};
use codetether_editor::FileBuffer;
/// Open the current goal as an in-memory draft.
/// # Arguments
/// Receives editor state and the current session ID.
/// # Returns
/// Updates the view, or displays an error while preserving existing buffers.
/// # Examples
/// ```no_run
/// fn open<H: codetether_goal_editor::Host>(host: &mut H) {
///     codetether_goal_editor::open(host, "session");
/// }
/// ```
pub fn open<H: Host>(app: &mut H, id: &str) {
    if app.buffer().is_some() || app.approval_open() {
        app.status("Close the current editor before editing the goal".into());
        return;
    }
    match H::Journal::read(id) {
        Ok(state) => {
            let Some(goal) = state.goal else {
                app.status("No goal to edit; use /goal set <objective> first".into());
                return;
            };
            app.set_buffer(Some(FileBuffer::draft(
                "Session goal (draft)",
                &goal.objective,
            )));
            app.set_draft(Some(Draft::new(id.into(), goal)));
            app.reset_scroll();
            app.clear_hover();
            app.clear_input();
            app.show_editor();
            app.status("Editing current goal — Ctrl+S save · Esc discard · Enter newline".into());
            app.redraw();
        }
        Err(error) => app.status(format!("Cannot open goal editor: {error}")),
    }
}
/// Discard the virtual draft and return to chat.
/// # Arguments
/// `app` supplies mutable editor state.
/// # Returns
/// Clears the draft, buffer, scroll and hover without updating the goal.
/// # Examples
/// ```no_run
/// fn close<H: codetether_goal_editor::Host>(host: &mut H) {
///     codetether_goal_editor::close(host);
/// }
/// ```
pub fn close<H: Host>(app: &mut H) {
    app.set_draft(None);
    app.set_buffer(None);
    app.reset_scroll();
    app.clear_hover();
    app.show_chat();
    app.redraw();
}
