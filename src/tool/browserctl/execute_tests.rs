use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn detect_claims_exact_approval_before_returning() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let args = json!({"action": "detect"});
    let blocked = crate::runtime_policy::evaluate_tool_invocation("browserctl", &args)
        .await
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    ApprovalStore::open_default()
        .expect("store")
        .approve(request_id, "test", "browser detection")
        .expect("approve");
    let mut approved = args;
    approved["approval_id"] = json!(request_id);

    let result = super::run(approved.clone()).await.expect("detect result");
    assert!(result.success);
    let replay = super::run(approved).await.expect("replay result");
    assert!(!replay.success);
    assert!(replay.output.contains("approval"));
}
