//! Deferred continuation waits for ownership and uses the newly persisted goal.

use super::fixture;
use crate::provider::ProviderRegistry;
use std::{path::Path, sync::Arc};

#[tokio::test]
async fn live_goal_commands_resume_only_after_cancellation_with_new_objective() {
    let (mut app, mut slot, runtime) = fixture::setup().await;
    let session = slot.take_for_prompt().unwrap();
    app.state.processing = true;
    fixture::submit(
        &mut app,
        &mut slot,
        &runtime,
        "/goal override new user objective",
    )
    .await;
    let registry = Some(Arc::new(ProviderRegistry::new()));
    super::super::drain(
        &mut app,
        Path::new("."),
        &mut slot,
        &registry,
        &None,
        &runtime,
    )
    .await;
    assert!(slot.borrow().is_none());
    assert!(app.state.main_inflight_prompt.is_none());
    slot.restore(session);
    app.state.processing = false;
    super::super::drain(
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
            .contains("new user objective")
    );
    assert!(!super::super::pending::take(slot.view().id()));
    runtime.shutdown().await;
}
