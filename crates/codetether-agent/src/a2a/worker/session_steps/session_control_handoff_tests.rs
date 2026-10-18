//! Worker handoff retains ownership; rejected duplicate entry cannot save history.

use super::{Mock, Scenario, run};
use crate::session::{Session, runtime::RunGuard};
use std::sync::Arc;

#[tokio::test]
async fn worker_handoff_keeps_parent_owner_and_applies_steering() {
    let mut session = Session::new().await.unwrap();
    let mut guard = RunGuard::open(&session.id).unwrap();
    let provider = Arc::new(Mock::new(&session.id, Scenario::Steering));
    let result = guard
        .run(run(&mut session, Arc::clone(&provider), 3))
        .await
        .unwrap();
    assert!(!result.budget_exhausted);
    assert_eq!(provider.outcome(), (2, true));
    assert!(RunGuard::open(&session.id).is_err());
    drop(guard);
    assert!(RunGuard::open(&session.id).is_ok());
}

#[tokio::test]
async fn rejected_independent_worker_does_not_record_a_halt() {
    let mut session = Session::new().await.unwrap();
    let _guard = RunGuard::open(&session.id).unwrap();
    let provider = Arc::new(Mock::new(&session.id, Scenario::Steering));
    let before = session.messages.len();
    assert!(run(&mut session, Arc::clone(&provider), 3).await.is_err());
    assert_eq!(session.messages.len(), before);
    assert_eq!(provider.outcome().0, 0);
}
