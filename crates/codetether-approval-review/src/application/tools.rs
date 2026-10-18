//! The reviewer's default catalog with shared read-only restrictions applied.
use codetether_application_registry::{Application, ToolRegistry};
/// Build the default tool registry and remove every shared-policy mutator.
/// # Arguments
/// The application catalog and authorization are selected by `H`.
/// # Returns
/// A restricted registry shared by direct and background reviewer execution.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_registry::Application>() {
/// let tools = codetether_approval_review::application::read_only_tools::<H>();
/// assert!(tools.get("write").is_none());
/// # }
/// ```
pub fn read_only_tools<H: Application>() -> ToolRegistry<H> {
    let mut registry = ToolRegistry::with_defaults();
    codetether_swarm_core::tool_policy::restrict_registry(&mut registry, true);
    registry
}
