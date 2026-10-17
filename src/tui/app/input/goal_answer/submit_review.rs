//! Present the durable answer-review transition after an explicit question route.

use crate::session::runtime::goals::answer_review;
use crate::tui::app::{session_runtime::TuiSessionHandle, state::App};

pub(super) async fn begin(
    app: &mut App,
    id: &str,
    runtime: &TuiSessionHandle,
    question: &str,
    input: &str,
) -> bool {
    match answer_review::begin(id, question).await {
        Ok(false) => false,
        Ok(true) => {
            runtime.request_cancel_current();
            app.state.push_history(input.to_string());
            app.state.clear_input();
            app.state.answer_review_yes = false;
            app.state.status = "Goal paused — answering your question before continuing".into();
            app.state.needs_redraw = true;
            true
        }
        Err(error) => {
            runtime.request_cancel_current();
            app.state.status = format!("Cannot safely pause goal: {error}");
            true
        }
    }
}
