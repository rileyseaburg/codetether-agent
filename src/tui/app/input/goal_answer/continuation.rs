//! Recognize explicit human continuation before treating input as a side question.

#[path = "continuation_release.rs"]
mod release;
use crate::tui::app::{
    session_runtime::{SessionSlot, TuiSessionHandle},
    state::App,
};
use release::release;

fn requested(input: &str) -> bool {
    matches!(
        input.trim().to_ascii_lowercase().as_str(),
        "continue" | "coninue" | "resume" | "/continue"
    )
}

pub(super) async fn intercept(
    app: &mut App,
    slot: &SessionSlot,
    runtime: &TuiSessionHandle,
) -> bool {
    if !requested(&app.state.input) {
        return false;
    }
    match release(slot.view().id()).await {
        Ok(false) => return false,
        Ok(true) => {
            app.state.push_history(app.state.input.trim().to_string());
            app.state.clear_input();
            app.state.answer_review_yes = false;
            super::commands::restart(app, slot.view().id(), runtime);
            app.state.status = "Goal continuation requested".into();
        }
        Err(error) => app.state.status = format!("Cannot safely resume goal: {error}"),
    }
    app.state.needs_redraw = true;
    true
}

#[cfg(test)]
#[path = "continuation_resume_tests.rs"]
mod resume_tests;
#[cfg(test)]
#[path = "continuation_tests.rs"]
mod tests;
