//! Persist satisfaction decisions before permitting any goal resumption.

use crate::session::tasks::{AnswerReviewAction, runtime::answer_review};
use crate::tui::app::{session_runtime::SessionSlot, state::App};
use anyhow::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

pub(super) async fn handle(
    app: &mut App,
    slot: &SessionSlot,
    key: KeyEvent,
) -> Result<Option<bool>> {
    let Some(review) = answer_review::read(slot.view().id())?
        .answer_review
        .filter(|review| review.ready)
    else {
        return Ok(None);
    };
    if matches!(key.code, KeyCode::PageUp | KeyCode::PageDown)
        || (matches!(key.code, KeyCode::Char('c' | 'q'))
            && key.modifiers.contains(KeyModifiers::CONTROL))
    {
        return Ok(None);
    }
    let Some(yes) = super::answer_review_choice::choose(&mut app.state.answer_review_yes, key)
    else {
        app.state.needs_redraw = true;
        return Ok(Some(false));
    };
    let decision = if yes {
        AnswerReviewAction::Satisfied
    } else {
        AnswerReviewAction::Unsatisfied
    };
    answer_review::record(slot.view().id(), &review.goal_id, &review.id, decision).await?;
    app.state.answer_review_yes = false;
    app.state.clear_input();
    app.state.status = if yes {
        "Answer accepted — resuming goal".into()
    } else {
        "Goal stays paused — ask a follow-up question".into()
    };
    app.state.needs_redraw = true;
    Ok(Some(yes))
}

#[cfg(test)]
#[path = "answer_review_key_tests.rs"]
mod tests;
