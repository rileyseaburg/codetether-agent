use std::path::Path;
/// Session-specific operations layered over the application catalog factory.
/// # Implementors
/// Applications supply MCP/command registration and prior-context access policy.
pub trait Factory: crate::composition::Factory {
    /// Register the configured application MCP connection.
    fn application_mcp(registry: &mut Self::Registry);
    /// Replace command tools with a shared process registry rooted at `cwd`.
    fn commands(registry: &mut Self::Registry, cwd: &Path);
    /// Remove tools that can access prior session context.
    fn remove_prior_context(registry: &mut Self::Registry);
}
