//! Native prompt composition, input enrichment and context completion.
use super::SessionView;
use codetether_agent_completion::Request;
use codetether_provider_core::CompletionResponse;
use serde_json::Value;
use std::path::Path;
/// Context services that still depend on the application's session machinery.
/// # Implementors
/// Preserve goal/peer prompt composition and the native completion options.
#[async_trait::async_trait]
pub trait Context: SessionView {
    /// Compose the agent persona with current session instructions.
    fn compose(base: &str, session: &Self::Session) -> String;
    /// Enrich parsed tool input with workspace and access-policy metadata.
    fn enrich(input: &Value, cwd: &Path, session: &Self::Session) -> Value;
    /// Complete using the existing context-aware provider path.
    /// # Errors
    /// Returns native context or provider errors.
    async fn complete(request: Request<'_, Self::Session>) -> anyhow::Result<CompletionResponse>;
}
