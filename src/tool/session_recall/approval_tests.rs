//! Evidence recall claims approval before local index access.

#[path = "approval_test_provider.rs"]
mod provider;

use super::SessionRecallTool;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::rlm::RlmConfig;
use crate::tool::Tool;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn evidence_recall_claims_exact_approval_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let tool = SessionRecallTool::new(
        Arc::new(provider::UnusedProvider),
        "unused".into(),
        RlmConfig::default(),
    );
    let mut args = json!({"query": "approval-bound evidence", "mode": "evidence"});
    let blocked = tool.execute(args.clone()).await.expect("blocked");
    assert!(!blocked.success);
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "evidence recall")
        .expect("approve");
    args["approval_id"] = json!(id);

    let recalled = tool.execute(args.clone()).await.expect("recall");
    assert!(!recalled.output.contains("TOOL_APPROVAL_REQUIRED"));
    let replay = tool.execute(args).await.expect("replay");
    assert!(!replay.success);
}
