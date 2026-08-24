//! Compatibility-alias approval receipt tests.

use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[path = "network_parse_claim_tests.rs"]
mod parse_claim;

#[tokio::test]
async fn alias_receipt_claims_once_under_the_reviewed_identity() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = super::test_env::Network::set("1");
    let mut args = json!({"prompt": "draw a secure boundary"});
    let scope = crate::runtime_policy::invocation_scope::for_tool("imagegen", &args);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("imagegen", scope.action, &scope.resource, "alias test")
        .expect("request");
    store
        .approve(&request.id, "test", "allow once")
        .expect("approve");
    args["approval_id"] = json!(request.id);

    let first = crate::tool::alias::scoped(
        "imagegen",
        super::invocation("image_gen", &args),
    )
    .await;
    assert!(first.is_none(), "approved alias should claim once");
    let replay = crate::tool::alias::scoped(
        "imagegen",
        super::invocation("image_gen", &args),
    )
    .await;
    assert!(replay.is_some(), "claimed alias receipt must reject replay");
}