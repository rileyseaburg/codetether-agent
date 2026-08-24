use super::{ApprovalStore, EnvGuard, guard, lock_env, policy_args};

#[tokio::test]
async fn missing_executable_does_not_consume_approval() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    let command = "codetether-definitely-missing-mcp-executable";
    let args = policy_args(command, &[], None);
    let scope = crate::runtime_policy::invocation_scope::for_tool("mcp", &args);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("mcp", scope.action, &scope.resource, "preflight")
        .expect("request");
    store
        .approve(&request.id, "test", "approved")
        .expect("approve");

    let error = guard(command, &[], Some(&request.id))
        .await
        .expect_err("missing executable");
    assert!(error.to_string().contains("executable not found"));
    assert!(
        store
            .verify(&request.id, "mcp", scope.action, &scope.resource,)
            .is_ok()
    );
}
