//! Command-prefix grants cannot authorize sandbox escalation.

use super::Registry;
use crate::tool::Tool;
use crate::tool::exec_command::ExecCommandTool;
use serde_json::json;
use std::sync::Arc;

#[path = "direct_escalation_tests.rs"]
mod direct;

#[tokio::test]
async fn command_prefix_grant_cannot_authorize_direct_escalation() {
    let _lock = crate::approval::test_env::lock_env();
    let workspace = std::env::current_dir().expect("workspace");
    let workspace = workspace.to_str().expect("workspace path");
    crate::approval::session_command_grants::remember_scoped_request_in(
        "prefix-request",
        vec!["true".into()],
        Some("session-1"),
        Some(workspace),
    );
    crate::approval::session_command_grants::grant_for_request("prefix-request");
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), None);
    let mut args = json!({
            "cmd": "true",
            "sandbox_permissions": "require_escalated",
            "__ct_session_id": "session-1",
            "__ct_parent_workspace": workspace,
        });
    crate::tool::network_access::bind_trusted(&mut args, false);
    let result = tool
        .execute(args)
        .await
        .unwrap();

    assert!(!result.success);
}