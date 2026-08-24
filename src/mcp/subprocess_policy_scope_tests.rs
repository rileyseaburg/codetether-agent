use super::{ApprovalStore, EnvGuard, guard, lock_env, policy_args};

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn approval_preserves_subprocess_argument_boundaries() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    if let Some(reason) = crate::tool::sandbox::unavailable_reason() {
        panic!("mandatory sandbox unavailable: {reason}");
    }
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let reviewed = policy_args("sh", &["alpha beta"], None);
    let blocked = crate::runtime_policy::evaluate_tool_invocation("mcp", &reviewed)
        .await
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    store
        .approve(request_id, "test", "argument boundary review")
        .expect("approve");

    assert!(
        guard("sh", &["alpha", "beta"], Some(request_id))
            .await
            .is_err()
    );
    assert!(guard("sh", &["alpha beta"], Some(request_id)).await.is_ok());
}
