//! Runtime-context authority tests.

use super::enrich_tool_input_with_runtime_context;
use serde_json::json;
use std::path::Path;

#[test]
fn runtime_identity_overrides_model_supplied_values() {
    let input = json!({
        "__ct_session_id": "spoofed",
        "__ct_agent_name": "spoofed",
    });
    let result = enrich_tool_input_with_runtime_context(
        &input,
        Path::new("/workspace"),
        None,
        "real-session",
        "real-agent",
        None,
    );
    assert_eq!(result["__ct_session_id"], "real-session");
    assert_eq!(result["__ct_agent_name"], "real-agent");
}

#[test]
fn exec_workdir_is_resolved_against_the_session_workspace() {
    let result = enrich_tool_input_with_runtime_context(
        &json!({ "workdir": "repo" }),
        Path::new("/workspace"),
        None,
        "session",
        "agent",
        None,
    );
    assert_eq!(result["workdir"], "/workspace/repo");
}
