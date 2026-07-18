use super::Args;
use serde_json::json;

#[test]
fn defaults_to_full_parent_history() {
    let args: Args = serde_json::from_value(json!({
        "task_name":"worker", "message":"work"
    }))
    .unwrap();
    assert_eq!(args.resolved_fork_turns(), "all");
}

#[test]
fn retains_hidden_fork_context_compatibility() {
    let args: Args = serde_json::from_value(json!({
        "message":"work", "fork_context":false
    }))
    .unwrap();
    assert_eq!(args.resolved_fork_turns(), "none");
}
