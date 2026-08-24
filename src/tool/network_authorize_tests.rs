use super::invocation;
use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use serde_json::json;

#[tokio::test]
async fn direct_network_invocation_requires_real_global_authority_and_approval() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("tempdir");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({"url": "https://example.com"});
    args[crate::tool::network_access::FIELD] = json!(true);

    let denied = invocation("webfetch", &args).await.expect("network denial");
    assert_eq!(denied.metadata["policy_reason"], "network_disabled");
    assert!(denied.metadata.get("approval_request_id").is_none());

    let _network = crate::tool::network_access::test_env::Network::set("1");
    let review = invocation("webfetch", &args).await.expect("approval required");
    assert_eq!(review.metadata["policy_outcome"], "require_approval");
    assert!(review.metadata["approval_request_id"].is_string());
}