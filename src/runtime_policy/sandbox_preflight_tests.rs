use super::decision_for_state;
use crate::config::SandboxMode;
use crate::runtime_policy::{DecisionReason, ToolPolicyOutcome};

#[test]
fn mutating_bash_requires_approval_when_sandbox_is_unavailable() {
    let decision = decision_for_state(
        "bash",
        "printf ok > file",
        SandboxMode::WorkspaceWrite,
        Some("bwrap_smoke_failed"),
        false,
    )
    .expect("decision");
    assert_eq!(decision.outcome, ToolPolicyOutcome::RequireApproval);
    assert_eq!(decision.reason, DecisionReason::SandboxUnavailable);
}

#[test]
fn read_only_bash_ignores_unavailable_sandbox() {
    assert!(
        decision_for_state(
            "bash",
            "date",
            SandboxMode::WorkspaceWrite,
            Some("bwrap_smoke_failed"),
            false,
        )
        .is_none()
    );
}
