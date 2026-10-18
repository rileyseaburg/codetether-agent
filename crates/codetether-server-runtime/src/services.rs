//! Startup services before task registries and state are finalized.
use crate::Parts;
use codetether_audit::AuditLog;
use codetether_bus_core::AgentBus;
use codetether_config::Config;
use codetether_http_auth::AuthState;
use codetether_k8s::K8sManager;
use codetether_plugin_verification::{PluginRegistry, hash_bytes};
use codetether_task_queue::KnativeTaskQueue;
use codetether_tool_discovery::ToolRegistry;
use std::sync::Arc;
pub(crate) struct Services<R> {
    pub config: Config,
    pub cognition: Arc<R>,
    pub audit_log: AuditLog,
    pub k8s: Arc<K8sManager>,
    pub auth_state: AuthState,
    pub bus: Arc<AgentBus>,
    pub plugin_registry: PluginRegistry,
}
impl<R> Services<R> {
    pub fn into_parts(self, version: &str) -> Parts<R> {
        Parts {
            config: Arc::new(self.config),
            cognition: self.cognition,
            audit_log: self.audit_log,
            k8s: self.k8s,
            auth: self.auth_state.clone(),
            bus: self.bus,
            knative_tasks: KnativeTaskQueue::new(),
            tool_registry: ToolRegistry::new(),
            plugin_registry: self.plugin_registry,
            server_fingerprint: hash_bytes(version.as_bytes()),
        }
    }
}
