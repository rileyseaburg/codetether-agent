//! Registry pruning for autonomous and read-only agents.
use codetether_tool_core::readonly::is_read_only;

/// Registry operations required by autonomous tool filtering.
/// # Implementors
/// Native registries supply their existing tool identifiers and removal operation.
pub trait Registry {
    /// List registered tool identifiers.
    fn list(&self) -> Vec<&str>;
    /// Remove a registered tool if present.
    fn unregister(&mut self, name: &str);
}
/// Apply the autonomous and optional read-only tool policy.
/// # Arguments
/// Receives the registry and whether execution is restricted to read-only tools.
/// # Returns
/// Removes disallowed tools in place using the shared classifier.
/// # Examples
/// ```no_run
/// fn restrict<R: codetether_swarm_core::tool_policy::Registry>(registry: &mut R) {
///     codetether_swarm_core::tool_policy::restrict_registry(registry, true);
/// }
/// ```
pub fn restrict_registry(registry: &mut impl Registry, read_only: bool) {
    if !read_only {
        registry.unregister("question");
        return;
    }
    let ids: Vec<String> = registry.list().into_iter().map(str::to_string).collect();
    for id in ids {
        if !is_read_only(&id) {
            registry.unregister(&id);
        }
    }
}
