//! Tests for zero-budget persona throttling.

use std::time::Duration;

use super::PersonaPolicy;
use super::tests_request::create;
use super::tests_support::runtime_with_policy;

/// A policy that can never afford a thought.
fn zero_budget() -> PersonaPolicy {
    PersonaPolicy {
        max_spawn_depth: 2,
        max_branching_factor: 2,
        token_budget_per_minute: 0,
        compute_ms_per_minute: 0,
        idle_ttl_secs: 3_600,
        share_memory: false,
        allowed_tools: Vec::new(),
    }
}

#[tokio::test]
async fn zero_budget_persona_is_skipped() {
    let runtime = runtime_with_policy(zero_budget());
    let persona = runtime
        .create_persona(create("budget-test", "tester", "test budgets"))
        .await
        .expect("should create persona");
    assert_eq!(persona.tokens_this_window, 0);

    runtime.start(None).await.expect("should start");
    tokio::time::sleep(Duration::from_millis(50)).await;
    runtime.stop(None).await.expect("should stop");

    let persona = runtime.get_persona("budget-test").await.unwrap();
    assert_eq!(persona.thought_count, 0);
    assert!(persona.budget_paused);
}
