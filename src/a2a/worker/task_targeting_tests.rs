//! Tests for explicit task-targeting rules.

use serde_json::json;

use super::task_targeting::explicitly_targets_agent;

#[test]
fn targeted_only_accepts_matching_metadata_target() {
    let task = json!({"id":"t1", "metadata":{"target_agent_name":"review-worker"}});
    assert!(explicitly_targets_agent(&task, "review-worker"));
}

#[test]
fn targeted_only_accepts_matching_nested_task_target() {
    let task = json!({"task":{"id":"t1", "target_agent_name":"review-worker"}});
    assert!(explicitly_targets_agent(&task, "review-worker"));
}

#[test]
fn targeted_only_rejects_untargeted_or_mismatched_tasks() {
    assert!(!explicitly_targets_agent(
        &json!({"id":"t1"}),
        "review-worker"
    ));
    assert!(!explicitly_targets_agent(
        &json!({"id":"t2", "metadata":{"target_agent_name":"other-worker"}}),
        "review-worker",
    ));
}
