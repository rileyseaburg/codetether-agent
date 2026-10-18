//! Bind initialized services to native application request state.
use crate::cognition::CognitionRuntime;
use codetether_server_runtime::Parts;
pub(super) fn build(parts: Parts<CognitionRuntime>) -> crate::server::AppState {
    crate::server::AppState {
        config: parts.config,
        cognition: parts.cognition,
        audit_log: parts.audit_log,
        k8s: parts.k8s,
        auth: parts.auth,
        bus: parts.bus,
        knative_tasks: parts.knative_tasks,
        tool_registry: parts.tool_registry,
        plugin_registry: parts.plugin_registry,
        server_fingerprint: parts.server_fingerprint,
    }
}
