//! Display session work items without presenting them as the session goal.

use crate::session::tasks::{runtime::answer_review, task_block};
use crate::tui::{
    app::state::App,
    chat::message::{ChatMessage, MessageType},
};

pub(super) async fn show(app: &mut App, session_id: &str, rest: &str) {
    if !matches!(rest.trim(), "" | "list") {
        app.state.status = "Usage: /tasks [list] — /goal manages the objective".into();
        return;
    }
    match answer_review::read(session_id) {
        Ok(state) => {
            app.state
                .messages
                .push(ChatMessage::new(MessageType::System, task_block(&state)));
            app.state.status = "Session tasks (work items); use /goal for the objective".into();
            app.state.scroll_to_bottom();
        }
        Err(error) => app.state.status = format!("/tasks: {error}"),
    }
}
