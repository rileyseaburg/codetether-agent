//! Application audit log access for HTTP adapters.
use codetether_audit::AuditLog;

/// Application state exposing the shared audit history.
/// # Implementors
/// Implement this trait on the state used by the server's audit layer.
pub trait HttpState: Clone + Send + Sync + 'static {
    /// Return the existing log used for request recording and history queries.
    fn audit_log(&self) -> &AuditLog;
}
