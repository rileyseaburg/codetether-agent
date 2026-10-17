//! Intercept user questions before approval feedback or live-turn steering.

#[path = "submit_question.rs"]
mod question;
#[path = "submit_review.rs"]
mod review;
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
    let question = question::parse(&input).or_else(|| {
        answer_review::held(slot.view().id())
            .then(|| question::followup(&input))
            .flatten()
    });
    let Some(question) = question else {
        return false;
    };
    review::begin(app, slot.view().id(), runtime, question, &input).await
}

#[cfg(test)]
#[path = "submit_followup_tests.rs"]
mod followup_tests;
#[cfg(test)]
#[path = "submit_tests.rs"]
mod tests;
