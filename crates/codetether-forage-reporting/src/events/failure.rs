//! Forage failure reporting.
use super::*;
impl RunEvents<'_> {
    /// Publish and audit a failed opportunity execution.
    pub async fn failure(&self, item: &ForageOpportunity, exec_task_id: &str, err: &anyhow::Error) {
        let error_message = format!("{err:#}");
        let _ = self.bus_handle.send_task_update(
            exec_task_id,
            TaskState::Failed,
            Some(format!("execution failed: {error_message}")),
        );
        log_audit(
            AuditCategory::Cognition,
            "forage.execute",
            AuditOutcome::Failure,
            Some(json!({
                "engine": self.args.execution_engine,
                "score": item.score,
                "okr_title": item.okr_title,
                "key_result_title": item.key_result_title,
                "error": error_message,
            })),
            Some(item.okr_id),
            None,
        )
        .await;
    }
}
