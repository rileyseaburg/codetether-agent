//! Claim-before-dispatch coverage for direct collaboration paths.

use crate::approval::test_env::{ScopedEnv, lock_env};
use crate::config::AccessMode;
use crate::tool::{ToolRegistry, network_access};
use serde_json::json;

#[tokio::test]
async fn steering_and_queue_paths_claim_their_own_receipts() {
    let _lock = lock_env();
    let data = tempfile::tempdir().unwrap();
    let workspace = tempfile::tempdir().unwrap();
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let registry = ToolRegistry::with_defaults();
    for tool_name in ["followup_task", "send_message"] {
        let mut args = json!({
            "target":"missing-child", "message":"hello",
            "__ct_session_id":"parent-session",
            "__ct_parent_workspace":workspace.path(),
        });
        network_access::bind_trusted(&mut args, true);
        super::approval_tests::approve(tool_name, &mut args);
        let tool = registry.get(tool_name).unwrap();

        let first = tool.execute(args.clone()).await.unwrap();
        assert_ne!(
            first.metadata.get("error_code"),
            Some(&json!("APPROVAL_RECEIPT_REJECTED")),
            "{tool_name}: {first:?}"
        );
        let replay = tool.execute(args).await.unwrap();
        assert_eq!(
            replay.metadata.get("error_code"),
            Some(&json!("APPROVAL_RECEIPT_REJECTED")),
            "{tool_name}: {replay:?}"
        );
    }
}