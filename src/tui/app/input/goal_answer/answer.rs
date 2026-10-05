//! Deliver a held question only after the cancelled goal returns its session.

use crate::provider::ProviderRegistry;
use crate::session::tasks::{AnswerReviewAction, runtime::answer_review};
use crate::tui::app::{session_runtime::SessionSlot, state::App};
use std::sync::Arc;

pub(crate) async fn deliver(
    app: &mut App,
    slot: &SessionSlot,
    registry: &Option<Arc<ProviderRegistry>>,
) -> bool {
    if app.state.processing {
        return false;
    }
    let Some(session) = slot.borrow() else {
        return false;
    };
    let review = match answer_review::read(&session.id) {
        Ok(state) => state.answer_review,
        Err(error) => {
            app.state.status = format!("Cannot read answer hold: {error}");
            return true;
        }
    };
    let Some(review) = review.filter(|review| review.question.is_some()) else {
        return false;
    };
    let question = review.question.as_deref().unwrap_or_default();
    let answered = crate::tui::app::ask::run_ask(app, session, registry.as_ref(), question).await;
    let decision = if answered {
        AnswerReviewAction::Answered
    } else {
        AnswerReviewAction::Unsatisfied
    };
    if let Err(error) =
        answer_review::record(&session.id, &review.goal_id, &review.id, decision).await
    {
        app.state.status = format!("Answer hold could not be saved; goal remains stopped: {error}");
    } else if answered {
        app.state.answer_review_yes = false;
        app.state.status =
            "Are you satisfied with the answer? Select Yes to resume or No to ask more.".into();
        app.state.scroll_to_bottom();
    }
    app.state.needs_redraw = true;
    true
}

#[cfg(test)]
#[path = "answer_tests.rs"]
mod tests;
