//! Backend approval precedes cached-session access.

use super::ContextSummarizeTool;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::Tool;
use serde_json::json;

#[tokio::test]
async fn cached_summary_backend_claims_exact_approval_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({
        "start": 0,
        "end": 1,
        "__ct_session_id": "context-summary-approval-test",
        "__ct_parent_workspace": std::env::current_dir().expect("cwd"),
    });
    crate::tool::network_access::bind_trusted(&mut args, true);
    let tool = ContextSummarizeTool::cached_only();
    let blocked = tool.execute(args.clone()).await.expect("blocked");
    assert!(!blocked.success);
    let id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(id, "test", "cached summary")
        .expect("approve");
    args["approval_id"] = json!(id);

    let result = tool.execute(args.clone()).await.expect("approved");
    assert!(!result.output.contains("TOOL_APPROVAL_REQUIRED"));
    let replay = tool.execute(args).await.expect("replay");
    assert!(!replay.success);
    assert!(replay.output.contains("already consumed"));
}
