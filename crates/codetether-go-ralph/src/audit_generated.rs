//! Audit the generated PRD using the process-global audit log.

use codetether_okr::{Okr, OkrRun};
use codetether_ralph_core::Prd;
use serde_json::json;
use std::path::Path;
pub(crate) async fn record(task: &str, okr: &Okr, okr_run: &OkrRun, prd: &Prd, prd_path: &Path) {
    if let Some(audit) = codetether_audit::try_audit_log() {
        audit
            .log_with_correlation(
                codetether_audit::AuditCategory::Cognition,
                "go_ralph_prd_generated",
                codetether_audit::AuditOutcome::Success,
                Some("codetether-agent".to_string()),
                Some(json!({
                    "task": task,
                    "prd_path": prd_path.display().to_string(),
                    "stories": prd.user_stories.len(),
                    "feature": prd.feature,
                    "project": prd.project,
                })),
                Some(okr.id.to_string()),
                Some(okr_run.id.to_string()),
                None,
                okr_run.session_id.clone(),
            )
            .await;
    }
}
