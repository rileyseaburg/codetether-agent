use crate::approval::{ApprovalStore, test_env::ScopedEnv, test_env::lock_env};
use crate::config::{AccessMode, Config};
use serde_json::json;

#[tokio::test]
async fn web_receipt_is_network_scoped_and_denial_does_not_consume_it() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let _network = crate::tool::network_access::test_env::Network::set("1");
    let mut enabled = json!({
        "url": "https://example.com",
        "__ct_effective_network_allowed": true,
    });
    let scope = crate::runtime_policy::invocation_scope::for_tool("webfetch", &enabled);
    let store = ApprovalStore::open_default().expect("store");
    let request = store
        .create_request("webfetch", scope.action, &scope.resource, "network")
        .expect("request");
    store
        .approve(&request.id, "test", "allow")
        .expect("approve");
    enabled["approval_id"] = json!(request.id);
    let mut disabled = enabled.clone();
    disabled[crate::tool::network_access::FIELD] = json!(false);

    crate::tool::network_access::test_env::Network::update("0");
    assert!(
        crate::runtime_policy::evaluate_tool_invocation_with_config(
            &Config::default(),
            "webfetch",
            &disabled,
        )
        .is_some()
    );
    crate::tool::network_access::test_env::Network::update("1");
    assert!(
        store
            .verify(&request.id, "webfetch", scope.action, &scope.resource)
            .is_ok()
    );
    assert!(
        crate::runtime_policy::evaluate_tool_invocation_with_config(
            &Config::default(),
            "webfetch",
            &enabled,
        )
        .is_none()
    );
    crate::approval::use_once::claim("webfetch", &enabled).expect("backend claim");
}
