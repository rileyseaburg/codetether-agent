//! RLM claims approval before collecting requested paths.

#[path = "approval_test_provider.rs"]
mod provider;

use super::super::RlmTool;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::rlm::RlmConfig;
use crate::tool::Tool;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn path_collection_claims_exact_approval_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let tool = RlmTool::new(
        Arc::new(provider::UnusedProvider),
        "unused".into(),
        RlmConfig::default(),
    );
    let mut args = json!({
        "action": "summarize", "paths": ["target/missing-approval-test"],
        "__ct_session_id": "rlm-approval-test",
        "__ct_parent_workspace": std::env::current_dir().expect("cwd"),
    });
    crate::tool::network_access::bind_trusted(&mut args, true);
    let blocked = tool.execute(args.clone()).await.expect("blocked");
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "RLM collection")
        .expect("approve");
    args["approval_id"] = json!(id);

    let collection_error = tool.execute(args.clone()).await.expect("collect");
    assert!(!collection_error.output.contains("TOOL_APPROVAL_REQUIRED"));
    let replay = tool.execute(args).await.expect("replay");
    assert!(replay.output.contains("already consumed"));
}
