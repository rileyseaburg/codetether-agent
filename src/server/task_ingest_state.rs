//! Native queue and audit-log bindings for task-ingress HTTP requests.
impl codetether_task_ingest::http::HttpState for super::AppState {
    fn queue(&self) -> &codetether_task_queue::KnativeTaskQueue {
        &self.inner.knative_tasks
    }
    fn audit_log(&self) -> &codetether_audit::AuditLog {
        &self.inner.audit_log
    }
}
