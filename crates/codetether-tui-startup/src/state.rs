//! Loaded startup dependencies and resolved session outcome.
use codetether_config::Config;
use codetether_session_resume::SessionLoadOutcome;
use codetether_workspace_snapshot::WorkspaceSnapshot;
use std::sync::Arc;
/// Independent startup results retained until UI/session hydration.
/// # Examples
/// Obtain this structure with [crate::load]; each result is consumed by a later stage.
pub struct Startup<R, B, T> {
    /// Optional provider registry.
    pub registry: Option<Arc<R>>,
    /// Optional bridge to an external worker.
    pub worker_bridge: Option<B>,
    /// Result of loading an explicitly requested session.
    pub session_load: Option<anyhow::Result<T>>,
    /// Result of loading configuration.
    pub config: Option<anyhow::Result<Config>>,
    /// Workspace snapshot captured during loading.
    pub workspace: WorkspaceSnapshot,
}
/// Session selected for startup and the reason it was selected.
/// # Examples
/// Native resolvers construct this after loading an existing or fresh session.
pub struct Resolved<S> {
    /// Native session value.
    pub session: S,
    /// Resumption/fresh-session outcome for the startup UI.
    pub outcome: SessionLoadOutcome,
}
