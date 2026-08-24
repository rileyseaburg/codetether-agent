use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::{AccessMode, ApprovalPolicy, Config};
use serde_json::json;

#[test]
fn generic_receipt_is_consumed_when_policy_currently_allows() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({"value": "same invocation"});
    let scope = super::super::invocation_scope::for_tool("generic_mutator", &args);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request(
            "generic_mutator",
            scope.action,
            &scope.resource,
            "transition",
        )
        .expect("request");
    store
        .approve(&request.id, "test", "allow")
        .expect("approve");
    args["approval_id"] = json!(request.id);
    let mut permissive = Config::default();
    permissive.approval_policy = Some(ApprovalPolicy::Never);

    assert!(
        super::evaluate_tool_invocation_with_config(&permissive, "generic_mutator", &args,)
            .is_none()
    );
    assert!(
        super::evaluate_tool_invocation_with_config(&Config::default(), "generic_mutator", &args,)
            .is_some()
    );
}
