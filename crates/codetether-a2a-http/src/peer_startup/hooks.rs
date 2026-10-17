//! Application-specific card, router and persistence bindings.
use super::*;

/// Callbacks and version supplied by the application composing a peer.
///
/// # Examples
///
/// ```rust
/// use codetether_a2a_http::peer_startup::StartupHooks;
/// use codetether_a2a_core::application_card;
/// let hooks = StartupHooks {
///     build_card: |_, url, _| application_card::default_card(url, "1.0.0"),
///     build_router: |_, _| axum::Router::new(),
///     ledger_path: || None,
///     application_version: "1.0.0",
/// };
/// assert_eq!(hooks.application_version, "1.0.0");
/// ```
#[derive(Clone, Copy)]
pub struct StartupHooks {
    /// Construct the card after the effective listener URL is known.
    pub build_card: fn(&str, &str, Option<&str>) -> AgentCard,
    /// Build the application router after discovery has started.
    pub build_router: fn(AgentCard, Arc<AgentBus>) -> Router,
    /// Resolve the configured introduction-ledger location on demand.
    pub ledger_path: fn() -> Option<PathBuf>,
    /// Application version advertised in mDNS service metadata.
    pub application_version: &'static str,
}
