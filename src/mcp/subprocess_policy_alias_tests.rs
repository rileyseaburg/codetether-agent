use super::{ApprovalStore, EnvGuard, guard, lock_env, policy_args};
use crate::approval::ApprovalEvent;

#[tokio::test]
#[ignore = "requires enforced OS sandbox; run in the mandatory sandbox CI lane"]
async fn approved_mcp_bridge_alias_allows_subprocess_spawn_gate() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = EnvGuard::data_dir(data.path());
    if let Some(reason) = crate::tool::sandbox::unavailable_reason() {
        panic!("mandatory sandbox unavailable: {reason}");
    }
    let store = ApprovalStore::open(data.path().join("approvals")).expect("store");
    let args = policy_args("npx", &["server"], None);
    let blocked = crate::runtime_policy::evaluate_tool_invocation("mcp_bridge", &args)
        .await
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id")
        .to_string();
    store.approve(&request_id, "riley", "ok").expect("approve");
    assert!(guard("npx", &["server"], Some(&request_id)).await.is_ok());
    let requests = store
        .events()
        .expect("events")
        .into_iter()
        .filter(|event| matches!(event, ApprovalEvent::Request { .. }))
        .count();
    assert_eq!(requests, 1, "alias authorization must not orphan a request");
    assert!(guard("npx", &["server"], Some(&request_id)).await.is_err());
}
