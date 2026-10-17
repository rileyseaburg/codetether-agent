//! Task step budgets remain independent of automatic goal continuation.

use super::{Mock, Scenario, run};
use crate::session::runtime::goals;
use std::sync::Arc;

#[tokio::test]
async fn worker_hard_step_budget_does_not_report_an_unfinished_goal_as_complete() {
    let mut session = crate::session::tasks::answer_review_test_support::session().await;
    let provider = Arc::new(Mock::new(&session.id, Scenario::Goal));
    let result = run(&mut session, provider, 1).await.unwrap();
    assert!(result.budget_exhausted);
    let (_, state) = goals::current(&session.id).await.unwrap();
    assert!(state.goal.unwrap().status.is_active());
}
