//! Keep accepting corrections while an active goal continuation is persisted.

use super::{RunGuard, continuation, steering};

#[test]
fn journal_wait_does_not_close_active_goal_input() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .max_blocking_threads(1)
        .build()
        .unwrap()
        .block_on(async {
            let mut session = crate::session::tasks::answer_review_test_support::session().await;
            let _guard = RunGuard::open(&session.id).unwrap();
            let id = session.id.clone();
            let (release, wait) = std::sync::mpsc::channel();
            let (started, running) = tokio::sync::oneshot::channel();
            let blocker = tokio::task::spawn_blocking(move || {
                started.send(()).unwrap();
                wait.recv().unwrap();
            });
            running.await.unwrap();
            let mut next = Box::pin(continuation::after_message(&mut session));
            assert!(futures::poll!(next.as_mut()).is_pending());
            let accepted = steering::send(&id, "Use the cloned repositories")
                .await
                .unwrap();
            release.send(()).unwrap();
            assert!(accepted);
            assert!(matches!(
                next.await.unwrap(),
                continuation::Continuation::Goal
            ));
            blocker.await.unwrap();
            assert_eq!(steering::drain_into(&mut session), 1);
        });
}
