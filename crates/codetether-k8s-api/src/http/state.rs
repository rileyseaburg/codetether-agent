//! Manager, audit and error-policy access for Kubernetes routes.
use codetether_audit::AuditLog;
use codetether_k8s::K8sManager;
/// Services and error policy already held by the application.
/// # Implementors
/// Native request state supplies existing manager and audit references.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Kubernetes manager for queries and mutations.
    fn manager(&self) -> &K8sManager;
    /// Audit log used by mutation operations.
    fn audit_log(&self) -> &AuditLog;
    /// Preserve the application's existing error response policy.
    fn map_error(error: anyhow::Error) -> (axum::http::StatusCode, String);
}
