//! Application services used when deriving provider-step state.
use codetether_provider_core::ToolDefinition;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
/// Supplies the registry, session context and application schema policy.
/// # Implementors
/// Applications retain registry construction and session/task ownership.
pub trait Host {
    /// Application-owned executable registry type.
    type Registry;
    /// Workspace, prior-context permission and autonomous-session flag.
    fn context(&self) -> (PathBuf, bool, bool);
    /// Construct a workspace-scoped registry with the supplied permissions.
    fn registry(&self, cwd: &Path, prior_context: bool, autonomous: bool) -> Arc<Self::Registry>;
    /// Filter the registry's active executable schemas.
    fn active(&self, registry: &Self::Registry, provider: &str) -> Vec<ToolDefinition>;
    /// Select advertised schemas and apply runtime policy.
    fn advertised(
        &self,
        supported: bool,
        tools: &[ToolDefinition],
        provider: &str,
    ) -> Vec<ToolDefinition>;
    /// Append the session's live task prompt.
    fn compose(&self, prompt: &str) -> String;
}
