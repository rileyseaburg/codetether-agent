use super::{enabled, unapproved_escalation};
use crate::config::{Config, SandboxMode};
use serde_json::json;

#[test]
fn escalation_requires_policy_authority_before_skipping_sandbox() {
    let args = json!({"cmd": "cargo test", "sandbox_permissions": "require_escalated"});
    let mut guarded = Config::default();
    guarded.sandbox_mode = Some(SandboxMode::WorkspaceWrite);
    assert!(enabled(&guarded, "cargo test", &args));
    assert!(unapproved_escalation(&args));
}
