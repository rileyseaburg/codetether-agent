use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::{Tool, websearch::WebSearchTool};
use serde_json::json;

#[tokio::test]
async fn malformed_websearch_does_not_consume_approval() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let mut args = json!({"query": ""});
    let blocked = crate::runtime_policy::evaluate_tool_invocation("websearch", &args)
        .await
        .expect("approval required");
    let request_id = blocked.metadata["approval_request_id"]
        .as_str()
        .expect("request id");
    let store = ApprovalStore::open_default().expect("store");
    store
        .approve(request_id, "test", "empty query preflight")
        .expect("approve");
    args["approval_id"] = json!(request_id);

    let result = WebSearchTool::new()
        .execute(args.clone())
        .await
        .expect("websearch result");
    assert!(!result.success);
    assert!(result.output.contains("empty"));
    let scope = crate::runtime_policy::invocation_scope::for_tool("websearch", &args);
    store
        .verify(request_id, "websearch", scope.action, &scope.resource)
        .expect("malformed input must preserve approval");
}