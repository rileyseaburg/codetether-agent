//! Live bridge startup and ordered catalog attachment.
use codetether_bus_http::BusBridge;
use codetether_mcp_core::server_catalog::ServerCatalog;
use std::sync::Arc;

/// Start a bus bridge and attach its tools and resources to an MCP catalog.
/// # Arguments
/// * `slot` — Server-owned bridge slot, updated before catalog registration.
/// * `catalog` — Server catalog receiving bus-aware tools and resources.
/// * `bus_url` — Event-bus endpoint.
/// * `auth_token` — Optional authentication token used by the bridge.
/// # Returns
/// Completes once the live bridge is stored and both registrations finish.
/// # Panics
/// Panics if bridge startup requires a Tokio runtime and none is active.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let mut bridge = None;
/// let catalog = codetether_mcp_core::server_catalog::ServerCatalog::new();
/// codetether_mcp_bus::attach(&mut bridge, &catalog, "http://localhost:8001".into(), None).await;
/// assert!(bridge.is_some());
/// # }
/// ```
pub async fn attach(
    slot: &mut Option<Arc<BusBridge>>,
    catalog: &ServerCatalog,
    bus_url: String,
    auth_token: Option<String>,
) {
    let bridge = BusBridge::with_auth(bus_url, auth_token).spawn();
    *slot = Some(Arc::clone(&bridge));
    crate::register_tools(catalog, Arc::clone(&bridge)).await;
    crate::register_resources(catalog, Arc::clone(&bridge)).await;
}
