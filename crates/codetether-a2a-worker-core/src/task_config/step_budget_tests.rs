//! Existing worker step-budget regression cases.

use super::*;

fn metadata(json: serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    json.as_object().cloned().unwrap_or_default()
}

#[test]
fn defaults_when_nothing_configured() {
    assert_eq!(
        resolve_with_env(&metadata(serde_json::json!({})), None),
        DEFAULT_WORKER_MAX_STEPS
    );
}

#[test]
fn task_metadata_wins_over_env() {
    let m = metadata(serde_json::json!({"max_steps": 12}));
    assert_eq!(resolve_with_env(&m, Some("400")), 12);
    let m = metadata(serde_json::json!({"step_budget": "30"}));
    assert_eq!(resolve_with_env(&m, Some("400")), 30);
}

#[test]
fn env_used_when_metadata_absent_and_zero_is_rejected() {
    let m = metadata(serde_json::json!({}));
    assert_eq!(resolve_with_env(&m, Some("400")), 400);
    assert_eq!(resolve_with_env(&m, Some("0")), DEFAULT_WORKER_MAX_STEPS);
    assert_eq!(resolve_with_env(&m, Some("nope")), DEFAULT_WORKER_MAX_STEPS);
}
