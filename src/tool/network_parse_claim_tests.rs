//! Network arguments must deserialize before an approval receipt is claimed.

use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::ToolResult;
use serde::Deserialize;
use serde_json::{Value, json};

#[derive(Deserialize)]
struct RequiredInput {
    required: String,
}

async fn parse(args: Value) -> anyhow::Result<ToolResult> {
    let input: RequiredInput = crate::tool::network_access::args!("webfetch", args);
    Ok(ToolResult::success(input.required))
}

#[tokio::test]
async fn invalid_input_does_not_consume_approved_receipt() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = super::super::test_env::Network::set("1");
    let mut args = json!({"wrong": "field"});
    let scope = crate::runtime_policy::invocation_scope::for_tool("webfetch", &args);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("webfetch", scope.action, &scope.resource, "parse first")
        .expect("request");
    store.approve(&request.id, "test", "allow").expect("approve");
    args["approval_id"] = json!(request.id);

    assert!(parse(args).await.is_err());
    assert!(
        store.verify(&request.id, "webfetch", scope.action, &scope.resource).is_ok()
    );
}