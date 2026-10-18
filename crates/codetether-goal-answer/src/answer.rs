//! Deliver a held question only after the cancelled goal returns its session.
use crate::Journal;
use codetether_goal_core::AnswerReviewAction;
/// Answer the current held question using the available native session.
/// # Arguments
/// `app` supplies session availability, answer generation and journal operations.
/// # Returns
/// True when a hold was processed or a read error was presented.
/// # Examples
/// ```no_run
/// async fn answer<H: codetether_goal_answer::Delivery>(host: &mut H) {
///     codetether_goal_answer::deliver(host).await;
/// }
/// ```
pub async fn deliver<H: crate::Delivery>(app: &mut H) -> bool {
    if app.processing() {
        return false;
    }
    let Some(id) = app.available_session() else {
        return false;
    };
    let review = match H::Journal::read(&id) {
        Ok(state) => state.answer_review,
        Err(error) => {
            app.status(format!("Cannot read answer hold: {error}"));
            return true;
        }
    };
    let Some(review) = review.filter(|review| review.question.is_some()) else {
        return false;
    };
    let question = review.question.as_deref().unwrap_or_default();
    let answered = app.ask(question).await;
    let decision = if answered {
        AnswerReviewAction::Answered
    } else {
        AnswerReviewAction::Unsatisfied
    };
    if let Err(error) = H::Journal::record(&id, &review.goal_id, &review.id, decision).await {
        app.status(format!(
            "Answer hold could not be saved; goal remains stopped: {error}"
        ));
    } else if answered {
        app.answer_no();
        app.status(
            "Are you satisfied with the answer? Select Yes to resume or No to ask more.".into(),
        );
        app.scroll();
    }
    app.redraw();
    true
}
