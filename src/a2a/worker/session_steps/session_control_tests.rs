//! Worker behavior must use the same control owner as TUI and CLI execution.

#[path = "session_control_budget_tests.rs"]
mod budget;
#[path = "session_control_fixture.rs"]
mod fixture;
#[path = "session_control_handoff_tests.rs"]
mod handoff;
#[path = "session_control_runner.rs"]
mod runner;
#[path = "session_control_stop_tests.rs"]
mod stops;
#[path = "session_control_token_budget_tests.rs"]
mod token_budget;
use crate::session::{Session, tasks::runtime::answer_review};
use fixture::{Mock, Scenario};
use runner::run;
use std::sync::Arc;

#[tokio::test]
async fn worker_steering_continues_after_a_final_response() {
    let mut session = Session::new().await.unwrap();
    let provider = Arc::new(Mock::new(&session.id, Scenario::Steering));
    let result = run(&mut session, Arc::clone(&provider), 3).await.unwrap();
    assert_eq!(provider.outcome(), (2, true));
    assert!(!result.budget_exhausted);
    assert!(!answer_review::held(&session.id));
    assert!(!crate::session::runtime::CONTROL.cancel_session(&session.id));
}

#[tokio::test]
async fn worker_active_goal_continues_and_accounts_usage() {
    let mut session = crate::session::tasks::answer_review_test_support::session().await;
    let provider = Arc::new(Mock::new(&session.id, Scenario::Goal));
    let result = run(&mut session, Arc::clone(&provider), 3).await.unwrap();
    assert_eq!(provider.outcome(), (2, true));
    assert!(!result.budget_exhausted);
    let (_, state) = crate::session::runtime::goals::current(&session.id)
        .await
        .unwrap();
    let goal = state.goal.unwrap();
    assert_eq!(goal.objective, "Finish the goal");
    assert_eq!(goal.turns_used, 2);
    assert_eq!(goal.tokens_used, 2);
    assert!(goal.status.is_terminal());
}
