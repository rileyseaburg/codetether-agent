//! Interactive fork command guards, activation and persistence ordering.
use super::{Session, View, creation, notice};
/// Fork the active session, optionally dropping trailing user turns.
/// # Arguments
/// * `view` - Active chat view.
/// * `session` - Active session, replaced with the fresh child after copying.
/// * `rest` - Optional nonnegative number of turns to drop.
/// # Returns
/// Completes after status/view updates; storage failures are reported in the view.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_session_fork::interactive::Session, V: codetether_session_fork::interactive::View>(session: &mut S, view: &mut V) {
/// codetether_session_fork::interactive::execute(view, session, "0").await;
/// # }
/// ```
pub async fn execute<S: Session, V: View>(view: &mut V, session: &mut S, rest: &str) {
    if view.processing() {
        view.push_system_message(
            "Cannot fork while a response is in progress. Press Esc to cancel first.".into(),
        );
        return;
    }
    let drop_last_n: usize = match rest.trim() {
        "" => 0,
        s => match s.parse::<usize>() {
            Ok(v) => v,
            Err(_) => {
                view.set_status(
                    "Usage: /fork [N] (drop last N user turns from the fork; default 0)"
                        .to_string(),
                );
                return;
            }
        },
    };
    let Some((child, parent_id, tui_cut)) = creation::create(session, view, drop_last_n).await
    else {
        return;
    };
    let child_id = child.id().to_string();
    *session = child;
    session.attach_global_bus_if_missing();
    if let Err(error) = session.save().await {
        view.set_status(format!("Fork created but failed to persist: {error}"));
        return;
    }
    let forked_tui = view.messages()[..tui_cut].to_vec();
    view.set_messages(forked_tui);
    view.set_session_id(session.id().to_string());
    view.clear_streaming_text();
    view.clear_request_timing();
    view.scroll_to_bottom();
    view.show_chat();
    notice::show(view, &child_id, &parent_id, drop_last_n);
}
