//! Raw policy calls cannot forge effective network authority.

use crate::approval::test_env::lock_env;
use crate::config::Config;
use serde_json::json;

#[tokio::test]
async fn caller_supplied_network_field_cannot_override_global_denial() {
    let _lock = lock_env();
    let _network = crate::tool::network_access::test_env::Network::set("0");
    let args = json!({
        "url": "https://example.com",
        "__ct_effective_network_allowed": true,
    });
    let blocked = crate::runtime_policy::evaluate_tool_invocation_with_config(
        &Config::default(),
        "webfetch",
        &args,
    )
    .expect("global denial");
    assert_eq!(
        blocked.metadata.get("policy_reason"),
        Some(&json!("network_disabled"))
    );
}
