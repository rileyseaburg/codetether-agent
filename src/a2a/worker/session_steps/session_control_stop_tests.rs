//! Holds and cancellation stop workers before tools execute.

use super::{Mock, Scenario, run};
use crate::session::{
    Session,
    runtime::{CONTROL, goals},
};
use std::sync::Arc;

#[tokio::test]
async fn worker_cancellation_interrupts_an_inflight_provider() {
    let mut session = Session::new().await.unwrap();
    let provider = Arc::new(Mock::new(&session.id, Scenario::Cancel));
    let error = run(&mut session, Arc::clone(&provider), 3)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("cancelled"));
    assert_eq!(provider.outcome().0, 1);
    assert!(!CONTROL.cancel_session(&session.id));
}

#[tokio::test]
async fn worker_pause_and_answer_hold_prevent_inflight_tools() {
    for scenario in [Scenario::Pause, Scenario::Hold] {
        let mut session = crate::session::tasks::answer_review_test_support::session().await;
        let provider = Arc::new(Mock::new(&session.id, scenario));
        let error = run(&mut session, Arc::clone(&provider), 3)
            .await
            .unwrap_err();
        assert!(error.to_string().contains("paused"));
        assert_eq!(provider.outcome().0, 1);
        assert!(
            !session
                .messages
                .iter()
                .any(|m| matches!(m.role, crate::provider::Role::Tool))
        );
        let (_, state) = goals::current(&session.id).await.unwrap();
        assert!(!state.goal.unwrap().status.is_active());
        assert_eq!(
            state.answer_review.is_some(),
            matches!(scenario, Scenario::Hold)
        );
    }
}
