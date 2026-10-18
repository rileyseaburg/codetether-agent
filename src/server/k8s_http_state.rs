//! Native Kubernetes, audit and error-policy bindings.
impl codetether_k8s_api::http::HttpState for super::AppState {
    fn manager(&self) -> &codetether_k8s::K8sManager {
        &self.k8s
    }
    fn audit_log(&self) -> &codetether_audit::AuditLog {
        &self.audit_log
    }
    fn map_error(error: anyhow::Error) -> (axum::http::StatusCode, String) {
        super::internal_error(error)
    }
}
