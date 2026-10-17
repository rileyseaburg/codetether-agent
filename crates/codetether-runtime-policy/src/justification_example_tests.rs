use super::{PLACEHOLDER, UNCHANGED, retry};
use serde_json::json;

#[test]
fn example_echoes_args_and_drops_internal_keys() {
    let args = json!({"command": "cargo test", "_tool_call_id": "c1", "approval_id": "a"});
    let example = retry(&args);
    assert_eq!(example["command"], "cargo test");
    assert_eq!(example["justification"], PLACEHOLDER);
    assert!(example.get("_tool_call_id").is_none());
    assert!(example.get("approval_id").is_none());
}

#[test]
fn long_values_are_elided() {
    let args = json!({"patch": "x".repeat(500), "dry_run": false});
    let example = retry(&args);
    assert_eq!(example["patch"], UNCHANGED);
    assert_eq!(example["dry_run"], false);
}

#[test]
fn non_object_args_get_minimal_example() {
    assert_eq!(retry(&json!(null))["justification"], PLACEHOLDER);
}
