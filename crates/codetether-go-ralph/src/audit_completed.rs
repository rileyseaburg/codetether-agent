//! Audit terminal story counts and Ralph lifecycle state.

use codetether_okr::{Okr, OkrRun};
use codetether_ralph_core::RalphState;
use serde_json::json;
use std::path::Path;
#[allow(clippy::too_many_arguments)]
pub(crate) async fn record(
    okr: &Okr,
    okr_run: &OkrRun,
    state: &RalphState,
    prd_path: &Path,
    passed: usize,
    total: usize,
    all_passed: bool,
) {
    if let Some(audit) = codetether_audit::try_audit_log() {
        let outcome = if all_passed {
            codetether_audit::AuditOutcome::Success
        } else {
            codetether_audit::AuditOutcome::Failure
        };
        audit
            .log_with_correlation(
                codetether_audit::AuditCategory::Cognition,
                "go_ralph_completed",
                outcome,
                Some("codetether-agent".to_string()),
                Some(json!({
                    "prd_path": prd_path.display().to_string(),
                    "passed": passed,
                    "total": total,
                    "status": format!("{:?}", state.status),
                    "iterations": state.current_iteration,
                    "feature_branch": state.prd.branch_name,
                })),
                Some(okr.id.to_string()),
                Some(okr_run.id.to_string()),
                None,
                okr_run.session_id.clone(),
            )
            .await;
    }
}
