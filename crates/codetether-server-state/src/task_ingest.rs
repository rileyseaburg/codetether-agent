//! Native queue and audit-log bindings for task-ingress HTTP requests.
use crate::AppState;
impl<R: Send + Sync + 'static> codetether_task_ingest::http::HttpState for AppState<R> {
    fn queue(&self) -> &codetether_task_queue::KnativeTaskQueue {
        &self.inner.knative_tasks
    }
    fn audit_log(&self) -> &codetether_audit::AuditLog {
        &self.inner.audit_log
    }
}
