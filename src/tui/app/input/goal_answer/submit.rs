//! Intercept user questions before approval feedback or live-turn steering.

use crate::session::tasks::runtime::answer_review;
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};

pub(crate) async fn intercept(
    app: &mut App,
    slot: &SessionSlot,
    runtime: &TuiSessionHandle,
) -> bool {
    let input = app.state.input.trim();
    let question = if let Some(rest) = input.strip_prefix("/ask") {
        if !rest.starts_with(char::is_whitespace) {
            return false;
        }
        rest.trim()
    } else if input.starts_with('/') || input.starts_with('!') {
        return false;
    } else {
        input
    };
    if question.is_empty() {
        return false;
    }
    match answer_review::begin(slot.view().id(), question).await {
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

#[cfg(test)]
#[path = "submit_tests.rs"]
mod tests;
