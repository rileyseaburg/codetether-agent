//! Queue and audit-log access for task-ingress requests.
use codetether_audit::AuditLog;
use codetether_task_queue::KnativeTaskQueue;

/// Application services used to receive task events.
/// # Implementors
/// Implement this trait on the server's existing shared request state.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Return the task queue populated by received events.
    fn queue(&self) -> &KnativeTaskQueue;
    /// Return the audit log used to record ingress events.
    fn audit_log(&self) -> &AuditLog;
}
