use super::{OutputCallback, RunRequest};
use crate::{execution_policy::AutoApprove, session_protocol::SessionStepsOutcome};
use codetether_provider_core::{Provider, ToolDefinition};
use std::{future::Future, path::Path, sync::Arc};
/// Native tool construction and session execution.
/// # Implementors
/// Adapters retain tool authorization and the application session loop.
pub trait Execution: Send {
    /// Native executable registry after approval filtering.
    type Tools: Send + Sync;
    /// Build the filtered registry using the existing approval policy.
    fn tools(
        &self,
        provider: Arc<dyn Provider>,
        model: String,
        policy: AutoApprove,
        workspace: &Path,
        output: OutputCallback,
    ) -> Self::Tools;
    /// Read the executable registry's definitions.
    fn definitions(&self, tools: &Self::Tools) -> Vec<ToolDefinition>;
    /// Compose the workspace-specific system prompt.
    fn system_prompt(&self, workspace: &Path) -> String;
    /// Run the bounded worker session.
    /// # Errors
    /// Propagates session execution and persistence failures.
    fn run(
        &mut self,
        request: RunRequest<'_, Self::Tools>,
    ) -> impl Future<Output = anyhow::Result<SessionStepsOutcome>> + Send;
}
