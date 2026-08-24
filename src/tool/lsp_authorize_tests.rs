use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn direct_lsp_boundary_claims_exact_approval_once() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let args = json!({"action": "workspaceSymbol", "query": "Session"});
    let blocked = super::result(&args)
        .await
        .expect("policy result")
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(request_id, "test", "lsp server")
        .expect("approve");
    let mut approved = args;
    approved["approval_id"] = json!(request_id);

    assert!(super::result(&approved).await.expect("approved").is_none());
    let replay = super::result(&approved)
        .await
        .expect("replay result")
        .expect("replay blocked");
    assert!(!replay.success);
}
