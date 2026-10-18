//! Publish the native registry's concrete schemas for worker training capture.
use super::{Application, Catalog, ToolRegistry};
use codetether_bus_core::{BusHandle, tool_catalog::from_definitions};

/// Publish the default catalog's complete tool schemas on the agent bus.
///
/// # Arguments
/// * `handle` - Worker bus handle used to announce the catalog.
///
/// # Returns
/// The receiver count returned by the bus announcement.
///
/// # Examples
/// ```no_run
/// # fn example<C: codetether_native_registry::Catalog>(handle: &codetether_bus_core::BusHandle) {
/// let _receivers = codetether_native_registry::announce::<C>(handle);
/// # }
/// ```
pub fn announce<C: Catalog>(handle: &BusHandle) -> usize {
    let schemas = from_definitions(&ToolRegistry::<Application<C>>::new().definitions());
    let count = schemas.len();
    let receivers = handle.announce_tool_catalog(schemas);
    tracing::info!(
        target: "codetether_agent::a2a::worker::worker_tool_catalog",
        tools = count,
        "Announced worker tool catalog for training capture"
    );
    receivers
}
