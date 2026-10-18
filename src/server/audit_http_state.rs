//! Native application audit log binding for HTTP routes and middleware.
impl codetether_audit_http::http::HttpState for super::AppState {
    fn audit_log(&self) -> &codetether_audit::AuditLog {
        &self.audit_log
    }
}
