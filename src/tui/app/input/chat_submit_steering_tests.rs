//! A correction during an active goal must reach the run without cancellation.

use super::*;
use crate::session::runtime::{CONTROL, goals, steering};
use futures::FutureExt;

#[tokio::test]
async fn enter_while_goal_is_active_steers_without_stopping_it() {
    let _guard = QUEUE_TEST_LOCK.lock().await;
    let _ = prompt_queue::take();
    let mut app = App::default();
    app.state.view_mode = ViewMode::Chat;
    app.state.processing = true;
    app.state.input = "use the cloned repos".into();
    let session = crate::session::tasks::answer_review_test_support::session().await;
    let id = session.id.clone();
    let mut slot = SessionSlot::new(session);
    let (event_tx, _) = tokio::sync::mpsc::channel(8);
    let (notice_tx, _) = tokio::sync::mpsc::channel(8);
    let runtime = session_runtime::spawn(event_tx, notice_tx);
    assert!(runtime.activate_steering(&id));
    let mut execution = CONTROL.begin(&id).unwrap();
    super::super::handle_enter_chat(
        &mut app,
        std::path::Path::new("."),
        &mut slot,
        &None,
        &None,
        &runtime,
    )
    .await;
    assert!(execution.cancelled().now_or_never().is_none());
    assert!(app.state.input.is_empty());
    assert!(prompt_queue::take().is_none());
    let state = goals::current(&id).await.unwrap().1;
    assert!(state.answer_review.is_none());
    let goal = state.goal.unwrap();
    assert!(goal.status.is_active());
    assert_eq!(goal.objective, "Finish the goal");
    let session = slot.borrow_mut().unwrap();
    assert_eq!(steering::drain_into(session), 1);
    assert!(matches!(&session.messages.last().unwrap().content[0],
        crate::provider::ContentPart::Text {text} if text == "use the cloned repos"));
    runtime.clear_steering();
    runtime.shutdown().await;
}
