//! Readiness gating and persist-before-feedback handling for answer review.
use super::{Host, passthrough};
use anyhow::Result;
use codetether_goal_core::AnswerReviewAction;
use crossterm::event::KeyEvent;

/// Handle a key against the current durable answer review.
///
/// # Arguments
/// * `host` — Current session review, selection, and UI effects.
/// * `key` — Terminal input event.
/// # Returns
/// `None` allows normal input processing. `Some(true)` reports acceptance;
/// `Some(false)` reports navigation, ignored modal input, or rejection.
/// # Errors
/// Propagates review-read and decision-write failures before post-decision UI updates.
/// # Examples
/// ```no_run
/// # async fn example(host: &mut impl codetether_answer_review_view::interactive::Host, key: crossterm::event::KeyEvent) -> anyhow::Result<()> {
/// let _outcome = codetether_answer_review_view::interactive::handle(host, key).await?;
/// # Ok(()) }
/// ```
pub async fn handle(host: &mut impl Host, key: KeyEvent) -> Result<Option<bool>> {
    let Some(review) = host.read_review()?.filter(|review| review.ready) else {
        return Ok(None);
    };
    if passthrough::allowed(host.input(), key) {
        return Ok(None);
    }
    let Some(yes) = crate::choose(host.selected_yes(), key) else {
        host.redraw();
        return Ok(Some(false));
    };
    let decision = if yes {
        AnswerReviewAction::Satisfied
    } else {
        AnswerReviewAction::Unsatisfied
    };
    host.record(&review, decision).await?;
    *host.selected_yes() = false;
    host.clear_input();
    host.set_status(if yes {
        "Answer accepted — resuming goal".into()
    } else {
        "Goal stays paused — ask a follow-up question".into()
    });
    host.redraw();
    Ok(Some(yes))
}
