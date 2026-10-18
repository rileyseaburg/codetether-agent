//! Routes whose native runtime adapters are supplied by the application.
use axum::Router;
/// Native adapter route groups merged at their existing application positions.
/// # Examples
/// ```
/// use codetether_server_router::Extensions;
/// let routes = Extensions::<()> {
///     approval: Default::default(), sessions: Default::default(),
///     realtime: Default::default(), status: Default::default(),
///     models: Default::default(), completions: Default::default(),
///     workers: Default::default(), resume: Default::default(),
/// };
/// ```
pub struct Extensions<S: Clone + Send + Sync + 'static> {
    /// Approval operations.
    pub approval: Router<S>,
    /// Durable session operations.
    pub sessions: Router<S>,
    /// Active session realtime transport.
    pub realtime: Router<S>,
    /// Configuration, identity and verifier status.
    pub status: Router<S>,
    /// Provider-backed model discovery.
    pub models: Router<S>,
    /// Provider-backed chat completions.
    pub completions: Router<S>,
    /// Worker and mux runtime endpoints.
    pub workers: Router<S>,
    /// Workspace-scoped session recovery.
    pub resume: Router<S>,
}
