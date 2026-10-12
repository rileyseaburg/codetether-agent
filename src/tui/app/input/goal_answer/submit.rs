//! Intercept user questions before approval feedback or live-turn steering.

#[path = "submit_question.rs"]
mod question;
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
    let input = app.state.input.trim().to_string();
    match super::continuation::release_ask(app, slot.view().id()).await {
        Ok(true) => return false,
        Ok(false) => {}
        Err(error) => {
            app.state.status = format!("Cannot safely resume goal: {error}");
            app.state.needs_redraw = true;
            return true;
        }
    }
    if super::continuation::intercept(app, slot, runtime).await {
        return true;
    }
    let Some(question) = question::parse(&input) else {
        return false;
    };
    match answer_review::begin(slot.view().id(), question).await {
        Ok(false) => false,
        Ok(true) => {
            runtime.request_cancel_current();
            app.state.push_history(input);
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
