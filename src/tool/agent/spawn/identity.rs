//! Application identity lookups for shared atomic spawn admission.
use crate::tool::ToolResult;
#[path = "identity/host.rs"]
mod host;
pub(super) use codetether_spawn_policy::Reservation;
pub(super) async fn reserve(name: &str, owner: Option<&str>) -> Result<Reservation, ToolResult> {
    codetether_spawn_policy::reserve::<host::ApplicationHost>(name, owner).await
}
#[cfg(test)]
#[path = "identity/scope_tests.rs"]
mod scope_tests;
