//! Interactive undo guards, argument parsing and history mutation.
use super::{Session, View, notice, persistence};
/// Undo requested user turns in the session and displayed chat history.
/// # Arguments
/// * `view` - Existing interactive chat state.
/// * `session` - Existing session history and durable storage.
/// * `rest` - Empty for one turn, otherwise a positive integer.
/// # Returns
/// Completes after status/notice updates; persistence failures are reported in the view.
/// # Examples
/// ```no_run
/// # async fn example<V: codetether_turn_undo::command::View, S: codetether_turn_undo::command::Session>(view: &mut V, session: &mut S) {
/// codetether_turn_undo::command::execute(view, session, "1").await;
/// # }
/// ```
pub async fn execute<V: View, S: Session>(view: &mut V, session: &mut S, rest: &str) {
    if view.processing() {
        view.push_system_message(
            "Cannot undo while a response is in progress. Press Esc to cancel first.".into(),
        );
        return;
    }
    let n: usize = match rest.trim() {
        "" => 1,
        s => match s.parse::<usize>() {
            Ok(v) if v >= 1 => v,
            _ => {
                view.set_status(
                    "Usage: /undo [N] (N = how many turns to undo, default 1)".to_string(),
                );
                return;
            }
        },
    };
    let (messages, pages) = session.history();
    let undo_count = crate::truncate_last_turns(messages, pages, view.messages(), n);
    if undo_count == 0 {
        view.push_system_message("Nothing to undo.".into());
        return;
    }
    session.set_updated_at(chrono::Utc::now());
    view.clear_streaming_text();
    view.scroll_to_bottom();
    persistence::save(session, view, undo_count).await;
    notice::show(view, undo_count, n);
}
