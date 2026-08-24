//! Queue pruning after an external durable decision.

use crate::approval::{
    ApprovalStore, LiveApprovalRequest, test_env::ScopedEnv, test_env::lock_env,
};
use crate::config::AccessMode;

#[test]
fn externally_decided_request_is_pruned() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    super::super::reset();
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("bash", "execute", "bash:abc", "test")
        .expect("request");
    super::super::push(LiveApprovalRequest::new(
        request.id.clone(),
        "call".into(),
        "bash".into(),
        "execute".into(),
        "bash:abc".into(),
        "test".into(),
    ));
    assert!(super::super::active().is_some());
    store
        .approve(&request.id, "mcp", "external")
        .expect("approve");
    assert!(super::super::active().is_none());
}
