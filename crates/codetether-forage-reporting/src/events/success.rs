//! Forage success reporting.
use super::*;
impl RunEvents<'_> {
    /// Publish completion and audit the opportunity execution evidence.
    pub async fn success(
        &self,
        item: &ForageOpportunity,
        exec_task_id: &str,
        execution_outcome: ExecutionOutcome,
    ) {
        let _ = self.bus_handle.send_task_update(
            exec_task_id,
            TaskState::Completed,
            Some(execution_outcome.detail.clone()),
        );
        log_audit(
            AuditCategory::Cognition,
            "forage.execute",
            AuditOutcome::Success,
            Some(json!({
                "engine": self.args.execution_engine,
                "score": item.score,
                "okr_title": item.okr_title,
                "key_result_title": item.key_result_title,
                "detail": execution_outcome.detail,
                "changed_files": execution_outcome.changed_files,
            })),
            Some(item.okr_id),
            None,
        )
        .await;
    }
}
