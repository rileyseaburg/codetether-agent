//! Approval enforcement before any backend process is created.

use super::super::RustyRoadTool;
use crate::runtime_policy::{ToolKind, evaluate_tool_invocation_with_config as evaluate};
use crate::{
    approval::{
        ApprovalStore,
        test_env::{ScopedEnv, lock_env},
    },
    config::{AccessMode, Config},
    tool::Tool,
};
use serde_json::json;

#[tokio::test]
async fn rustyroad_ask_requires_justification_before_spawn() {
    let _lock = lock_env();
    let data = tempfile::tempdir().unwrap();
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let result = RustyRoadTool
        .execute(json!({"action":"list_tools", "cwd":data.path()}))
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("TOOL_JUSTIFICATION_REQUIRED"));
    assert_eq!(ToolKind::for_name("rustyroad"), ToolKind::Mutating);
}

#[test]
fn rustyroad_approval_covers_project_environment_and_arguments() {
    let _lock = lock_env();
    let data = tempfile::tempdir().unwrap();
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let store = ApprovalStore::open_default().unwrap();
    let config = Config::default();
    let mut input = json!({"action":"call_tool", "cwd":data.path(), "environment":"dev",
        "tool_name":"rustyroad_query", "arguments":{"sql":"SELECT 1"}, "justification":"Inspect fixture"});
    let blocked = evaluate(&config, "rustyroad", &input).unwrap();
    let id = blocked.metadata["approval_request_id"].as_str().unwrap();
    store.approve(id, "test", "fixture inspection").unwrap();
    input["approval_id"] = json!(id);
    assert!(evaluate(&config, "rustyroad", &input).is_none());
    for (field, value) in [
        ("environment", json!("prod")),
        ("cwd", json!("/different/project")),
        ("arguments", json!({"sql":"DELETE FROM users"})),
        ("tool_name", json!("rustyroad_migrate")),
    ] {
        let mut changed = input.clone();
        changed[field] = value;
        assert!(evaluate(&config, "rustyroad", &changed).is_some());
    }
}
