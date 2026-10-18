//! Initialized services supplied to native request-state construction.
use codetether_audit::AuditLog;
use codetether_bus_core::AgentBus;
use codetether_config::Config;
use codetether_http_auth::AuthState;
use codetether_k8s::K8sManager;
use codetether_plugin_verification::PluginRegistry;
use codetether_task_queue::KnativeTaskQueue;
use codetether_tool_discovery::ToolRegistry;
use std::sync::Arc;
/// Services created by startup for the application's concrete request state.
/// # Examples
/// ```no_run
/// # fn inspect<R>(parts: codetether_server_runtime::Parts<R>) {
/// let bus = parts.bus.clone();
/// let fingerprint = parts.server_fingerprint;
/// # }
/// ```
pub struct Parts<R> {
    /// Loaded application configuration.
    pub config: Arc<Config>,
    /// Configured cognition runtime.
    pub cognition: Arc<R>,
    /// Initialized audit history.
    pub audit_log: AuditLog,
    /// Kubernetes manager.
    pub k8s: Arc<K8sManager>,
    /// Mandatory authentication state.
    pub auth: AuthState,
    /// Process-wide communication bus.
    pub bus: Arc<AgentBus>,
    /// In-process task queue.
    pub knative_tasks: KnativeTaskQueue,
    /// Dynamic tool discovery registry.
    pub tool_registry: ToolRegistry,
    /// Native tool plugin registry.
    pub plugin_registry: PluginRegistry,
    /// Fingerprint derived from the executable's package version.
    pub server_fingerprint: String,
}

impl<R> Clone for Parts<R> {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            cognition: self.cognition.clone(),
            audit_log: self.audit_log.clone(),
            k8s: self.k8s.clone(),
            auth: self.auth.clone(),
            bus: self.bus.clone(),
            knative_tasks: self.knative_tasks.clone(),
            tool_registry: self.tool_registry.clone(),
            plugin_registry: self.plugin_registry.clone(),
            server_fingerprint: self.server_fingerprint.clone(),
        }
    }
}
