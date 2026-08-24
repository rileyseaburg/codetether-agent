use super::{decision, escalation};
use crate::config::{ApprovalPolicy, Config, SandboxMode};
use crate::runtime_policy::{RuntimeToolPolicy, ToolPolicyOutcome};
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

#[test]
fn on_failure_escalation_requires_approval() {
    let mut config = Config::default();
    config.approval_policy = Some(ApprovalPolicy::OnFailure);
    let policy = RuntimeToolPolicy::from_config(&config);
    let args = json!({"sandbox_permissions": "require_escalated"});

    let decision = escalation(&policy, "exec_command", &args).expect("decision");

    assert_eq!(decision.outcome, ToolPolicyOutcome::RequireApproval);
}

#[test]
fn never_escalation_is_denied() {
    let mut config = Config::default();
    config.approval_policy = Some(ApprovalPolicy::Never);
    let policy = RuntimeToolPolicy::from_config(&config);
    let args = json!({"sandbox_permissions": "require_escalated"});

    let decision = escalation(&policy, "exec_command", &args).expect("decision");

    assert_eq!(decision.outcome, ToolPolicyOutcome::Deny);
}
