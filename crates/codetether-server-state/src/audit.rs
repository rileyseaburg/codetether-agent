//! Native application audit log binding for HTTP routes and middleware.
use crate::AppState;
impl<R: Send + Sync + 'static> codetether_audit_http::http::HttpState for AppState<R> {
    fn audit_log(&self) -> &codetether_audit::AuditLog {
        &self.inner.audit_log
    }
}
