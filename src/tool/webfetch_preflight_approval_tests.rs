//! Invalid request preflight does not consume approved receipts.

use super::super::WebFetchTool;
use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::Tool;
use serde_json::json;

#[tokio::test]
async fn malformed_url_preserves_approved_receipt() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let mut args = json!({"url": ":not-a-url"});
    let scope = crate::runtime_policy::invocation_scope::for_tool("webfetch", &args);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("webfetch", scope.action, &scope.resource, "preflight")
        .expect("request");
    store
        .approve(&request.id, "test", "allow")
        .expect("approve");
    args["approval_id"] = json!(request.id);

    let error = WebFetchTool::new()
        .execute(args)
        .await
        .expect_err("invalid URL");
    assert!(error.to_string().contains("Invalid URL"));
    assert!(
        store
            .verify(&request.id, "webfetch", scope.action, &scope.resource)
            .is_ok()
    );
}
