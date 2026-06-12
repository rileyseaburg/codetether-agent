use super::decision;
use crate::config::{ApprovalPolicy, Config, SandboxMode};
use crate::runtime_policy::RuntimeToolPolicy;
use serde_json::json;

#[test]
fn approval_never_skips_sandbox_unavailable_preflight() {
    let mut config = Config::default();
    config.approval_policy = Some(ApprovalPolicy::Never);
    config.sandbox_mode = Some(SandboxMode::WorkspaceWrite);
    let policy = RuntimeToolPolicy::from_config(&config);
    let args = json!({"command": "printf ok > file"});

    assert!(decision(&policy, "bash", &args).is_none());
}
