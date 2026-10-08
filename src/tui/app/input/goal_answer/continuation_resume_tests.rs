//! Continuation waits for session ownership and resumes the persisted objective.
use crate::provider::ProviderRegistry;
use crate::session::tasks::runtime::answer_review;
use crate::tui::app::input::goal_answer::commands;
use std::{path::Path, sync::Arc};
#[path = "commands/tests/fixture.rs"]
mod fixture;

#[tokio::test]
async fn answer_review_continuation_resumes_only_after_session_returns() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let id = slot.view().id().to_string();
    answer_review::begin(&id, "Why?").await.unwrap();
    let session = slot.take_for_prompt().unwrap();
    app.state.processing = true;
    fixture::submit(&mut app, &mut slot, &runtime, "coninue").await;
    assert!(!answer_review::held(&id));
    let registry = Some(Arc::new(ProviderRegistry::new()));
    commands::drain(
        &mut app,
        Path::new("."),
        &mut slot,
        &registry,
        &None,
        &runtime,
    )
    .await;
    assert!(app.state.main_inflight_prompt.is_none());
    assert!(slot.borrow().is_none());
    slot.restore(session);
    app.state.processing = false;
    commands::drain(
        &mut app,
        Path::new("."),
        &mut slot,
        &registry,
        &None,
        &runtime,
    )
    .await;
    assert!(
        app.state
            .main_inflight_prompt
            .as_ref()
            .unwrap()
            .contains("Finish the goal")
    );
    assert!(slot.borrow().is_none());
    runtime.shutdown().await;
}
