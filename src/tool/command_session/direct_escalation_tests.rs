//! Direct callers cannot self-authorize sandbox escalation.

use super::{ExecCommandTool, Registry};
use crate::approval::{test_env::ScopedEnv, test_env::lock_env};
use crate::config::AccessMode;
use crate::tool::Tool;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn direct_exec_cannot_self_authorize_sandbox_escalation() {
    let _lock = lock_env();
    let data = tempfile::tempdir().expect("data");
    let _env = ScopedEnv::data_dir_with_access(data.path(), AccessMode::Ask);
    let mut args = json!({
        "cmd": "true",
        "sandbox_permissions": "require_escalated",
        "__ct_session_id": "direct-escalation-test",
        "__ct_parent_workspace": data.path(),
    });
    crate::tool::network_access::bind_trusted(&mut args, false);
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), None);
    let result = tool.execute(args).await.expect("approval result");
    assert!(!result.success);
    assert!(
        result.metadata["approval_request_id"].is_string(),
        "{}",
        result.output
    );
}