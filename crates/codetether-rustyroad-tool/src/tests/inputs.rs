//! Input validation without launching a backend.

use super::super::{invocation, params::Params};
use serde_json::json;

#[test]
fn rustyroad_requires_typed_action_project_and_environment() {
    for input in [
        json!({"action":"list_tools"}),
        json!({"action":"shell", "cwd":"."}),
        json!({"action":"list_tools", "cwd":".", "environment":"../prod"}),
        json!({"action":"call_tool", "cwd":".", "arguments": []}),
    ] {
        assert!(serde_json::from_value::<Params>(input).is_err());
    }
}

#[test]
fn rustyroad_defaults_to_dev_and_injects_matching_environment() {
    let project = tempfile::tempdir().unwrap();
    let mut params: Params = serde_json::from_value(json!({
        "action":"call_tool", "cwd":project.path(), "tool_name":"rustyroad_schema"
    }))
    .unwrap();
    assert_eq!(params.environment.as_str(), "dev");
    assert_eq!(
        invocation::prepare(&mut params).unwrap(),
        project.path().canonicalize().unwrap()
    );
    assert_eq!(params.arguments["env"], "dev");
}

#[test]
fn rustyroad_rejects_environment_overrides_and_invalid_calls() {
    let project = tempfile::tempdir().unwrap();
    for input in [
        json!({"action":"call_tool", "tool_name":"rustyroad_query", "arguments":{"env":"prod"}}),
        json!({"action":"call_tool", "tool_name":"other_tool"}),
        json!({"action":"call_tool"}),
        json!({"action":"list_tools", "arguments":{"env":"prod"}}),
        json!({"action":"list_tools", "tool_name":"rustyroad_config"}),
        json!({"action":"list_tools", "cwd":""}),
    ] {
        let mut input = input;
        input
            .as_object_mut()
            .unwrap()
            .entry("cwd")
            .or_insert(json!(project.path()));
        let mut params: Params = serde_json::from_value(input).unwrap();
        assert!(invocation::prepare(&mut params).is_err());
    }
}
