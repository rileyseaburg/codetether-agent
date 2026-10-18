//! Parent cancellation waits for cleanup and saves accepted steering.

use super::{RunGuard, steering, swarm_monitor};
use crate::session::tasks::answer_review_test_support;
use std::sync::atomic::{AtomicBool, Ordering};

#[tokio::test]
async fn cancellation_waits_for_delegated_cleanup() {
    let mut session = answer_review_test_support::session().await;
    let guard = RunGuard::open(&session.id).unwrap();
    let control = guard.handle();
    let cleaned = AtomicBool::new(false);
    control.cancel();
    let result = swarm_monitor::run(&mut session, &control, async {
        control.cancelled().await;
        tokio::task::yield_now().await;
        cleaned.store(true, Ordering::SeqCst);
        Ok(())
    })
    .await;
    assert!(result.is_err());
    assert!(cleaned.load(Ordering::SeqCst));
    assert!(
        !steering::send(&session.id, "after cancellation")
            .await
            .unwrap()
    );
}

#[tokio::test]
async fn final_input_is_stored_and_broadcast_without_closing_handoff_inbox() {
    let mut session = answer_review_test_support::session().await;
    let guard = RunGuard::open(&session.id).unwrap();
    let control = guard.handle();
    assert!(
        steering::send(&session.id, "Use the cloned repos")
            .await
            .unwrap()
    );
    swarm_monitor::run(&mut session, &control, async { Ok(()) })
        .await
        .unwrap();
    let mut cursor = 0;
    assert_eq!(control.inputs_since(&mut cursor).len(), 1);
    assert_eq!(
        session.messages.last().unwrap().role,
        crate::provider::Role::User
    );
    assert!(steering::send(&session.id, "between phases").await.unwrap());
}
