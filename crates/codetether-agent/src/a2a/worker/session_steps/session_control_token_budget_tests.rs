//! Accounting stops workers before tools when the user's token budget expires.

use super::{Mock, Scenario, run};
use crate::session::{
    runtime::goals,
    tasks::{GoalRuntimeUpdate, GoalStatus, TaskEvent},
};
use std::sync::Arc;

#[tokio::test]
async fn worker_token_budget_blocks_inflight_dispatch_without_completing_goal() {
    let mut session = crate::session::tasks::answer_review_test_support::session().await;
    let (log, _) = goals::current(&session.id).await.unwrap();
    log.append(&TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: chrono::Utc::now(),
        goal_id: "goal".into(),
        objective: None,
        status: None,
        token_budget: Some(1),
        token_delta: 0,
        elapsed_seconds: 0,
        continuation_delta: 0,
    }))
    .await
    .unwrap();
    let provider = Arc::new(Mock::new(&session.id, Scenario::Steering));
    let error = run(&mut session, Arc::clone(&provider), 3)
        .await
        .unwrap_err();
    assert!(error.to_string().contains("budget_limited"));
    assert_eq!(provider.outcome().0, 1);
    let (_, state) = goals::current(&session.id).await.unwrap();
    let goal = state.goal.unwrap();
    assert_eq!(goal.status, GoalStatus::BudgetLimited);
    assert_eq!(goal.objective, "Finish the goal");
    assert_eq!(goal.tokens_used, 2);
}
