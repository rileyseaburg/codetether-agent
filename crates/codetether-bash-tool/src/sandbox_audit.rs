//! Append the sandbox execution audit event when auditing is initialized.
use codetether_audit::{AuditCategory, try_audit_log};
use codetether_sandbox_policy::SandboxResult;
pub(super) async fn record(command: &str, sandbox_result: &anyhow::Result<SandboxResult>) {
    if let Some(audit) = try_audit_log() {
        let (outcome, detail) = super::sandbox_detail::detail(sandbox_result);
        audit
            .log(
                AuditCategory::Sandbox,
                format!(
                    "bash:{}",
                    codetether_util::truncate_bytes_safe(&command, 80)
                ),
                outcome,
                None,
                Some(detail),
            )
            .await;
    }
}
