//! Already-bound A2A transport routers.
use axum::Router;
/// A2A discovery and invocation routers for root and prefixed mounts.
/// # Examples
/// ```
/// let transports = codetether_server_router::Transports {
///     root: axum::Router::new(), nested: axum::Router::new(),
/// };
/// ```
pub struct Transports {
    /// A2A routes served at the application root.
    pub root: Router,
    /// A2A routes served beneath `/a2a`.
    pub nested: Router,
}
