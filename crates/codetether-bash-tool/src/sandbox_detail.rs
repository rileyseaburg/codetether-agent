//! Outcome and payload for sandbox audit records.
use codetether_audit::AuditOutcome;
use codetether_sandbox_policy::SandboxResult;
use serde_json::{Value, json};
pub(super) fn detail(sandbox_result: &anyhow::Result<SandboxResult>) -> (AuditOutcome, Value) {
    let (outcome, detail) = match sandbox_result {
        Ok(r) => (
            if r.success {
                AuditOutcome::Success
            } else {
                AuditOutcome::Failure
            },
            json!({
                "sandboxed": true,
                "exit_code": r.exit_code,
                "duration_ms": r.duration_ms,
                "violations": r.sandbox_violations,
                "unsafe_fallbacks": r.unsafe_fallbacks,
            }),
        ),
        Err(e) => (
            AuditOutcome::Failure,
            json!({ "sandboxed": true, "error": e.to_string() }),
        ),
    };

    (outcome, detail)
}
