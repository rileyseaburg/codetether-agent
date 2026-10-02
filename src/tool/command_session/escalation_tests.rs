//! Escalation requires a real, exact approved decision before process spawn.

#[path = "denied_execution_tests.rs"]
mod denied;
#[path = "approval_fixture.rs"]
mod fixture;
#[path = "approved_write_tests.rs"]
mod writes;

use super::Registry;
use crate::tool::Tool;
use crate::tool::exec_command::ExecCommandTool;
use serde_json::json;
use std::sync::Arc;

#[tokio::test]
async fn direct_exec_cannot_self_authorize_sandbox_escalation() {
    let tool = ExecCommandTool::new(Arc::new(Registry::default()), None);
    let result = tool
        .execute(json!({
            "cmd": "true",
            "sandbox_permissions": "require_escalated"
        }))
        .await
        .unwrap();
    assert!(!result.success);
    assert!(result.output.contains("requires an approved"));
}
