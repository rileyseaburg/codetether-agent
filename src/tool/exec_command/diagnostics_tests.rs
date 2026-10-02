//! Approval correlation is useful without dumping commands or environment secrets.

use serde_json::json;

#[test]
fn escalation_error_has_safe_copyable_binding_diagnostics() {
    let args = json!({"cmd": "TOKEN=private-test-sentinel cargo update",
        "approval_id": "approved-request", "sandbox_permissions": "require_escalated"});
    let result = super::escalation_error(&args);
    let details = &result.metadata["execution_diagnostics"];
    assert_eq!(details["approval_id"], "approved-request");
    assert!(
        details["approval_resource"]
            .as_str()
            .unwrap()
            .starts_with("exec_command:")
    );
    assert_eq!(details["effective_sandbox_mode"], "not-started");
    assert!(!result.output.contains("private-test-sentinel"));
    assert!(result.output.contains("approved-request"));
    assert!(result.output.contains("exec_command:"));
}

#[test]
fn effective_isolation_mode_is_reported_without_raw_arguments() {
    let args = json!({"cmd": "secret-test-sentinel"});
    let policy = crate::tool::sandbox::SandboxPolicy::default();
    let details = super::execution(&args, std::path::Path::new("/workspace"), Some(&policy));
    assert_eq!(details["effective_sandbox_mode"], "read-only");
    assert!(!details.to_string().contains("secret-test-sentinel"));
    let details = super::execution(&args, std::path::Path::new("/workspace"), None);
    assert_eq!(details["effective_sandbox_mode"], "unsandboxed");
}
