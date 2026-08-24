//! Preview execution consumes the same exact authority as reset.

use super::super::{Tool, UndoTool};
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::{AccessMode, Config, PermissionAction};
use serde_json::json;

#[tokio::test]
async fn undo_preview_is_blocked_then_executes_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let tool = UndoTool;
    let mut args = json!({"steps": 1, "preview": true});
    let mut config = Config::default();
    config
        .permissions
        .tools
        .insert("undo".into(), PermissionAction::Ask);
    let blocked =
        crate::runtime_policy::evaluate_tool_invocation_with_config(&config, "undo", &args)
            .expect("approval request");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "undo preview")
        .expect("approve");
    args["approval_id"] = json!(id);

    let preview = tool.execute(args.clone()).await.expect("preview");
    assert!(preview.success, "{}", preview.output);
    assert!(preview.output.starts_with("Would undo"));
    let replay = tool.execute(args).await.expect("replay");
    assert!(!replay.success);
}
