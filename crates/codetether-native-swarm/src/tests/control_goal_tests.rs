//! Delegated completions charge their canonical parent's existing goal.

use super::goal_fixture as answer_review_test_support;
use super::{control_test_provider::Mock, driver, image_test_state};
use codetether_native_goals::runtime as goals;
use codetether_native_session_runtime::RunGuard;
use codetether_tool_core::ToolResult;
use std::sync::{Arc, Mutex};

#[tokio::test]
async fn delegated_usage_preserves_parent_objective_and_counts_each_response() {
    let session = answer_review_test_support::session().await;
    let guard = RunGuard::open(&session.id).unwrap();
    let control = guard.handle();
    let provider = Arc::new(Mock {
        control: control.clone(),
        cancel: false,
        requests: Mutex::new(vec![]),
    });
    let mut state = control
        .scope(async { image_test_state::state(ToolResult::success("unused")) })
        .await;
    state.inner.provider = provider;
    state.inner.max_steps = 3;
    driver::execute(&mut state).await.unwrap();
    let (_, state) = goals::current(&session.id).await.unwrap();
    let goal = state.goal.unwrap();
    assert_eq!(goal.tokens_used, 10);
    assert_eq!(goal.objective, "Finish the goal");
    assert!(goal.status.is_active());
}
