//! Native filtered tool construction and bounded worker execution.
use super::{Application, Native, ToolRegistry, policy};
use codetether_a2a_worker_core::{
    execution_policy::AutoApprove, session_protocol::SessionStepsOutcome,
};
use codetether_native_providers::{Provider, ToolDefinition};
use std::{path::Path, sync::Arc};
impl<S, T, W> policy::Execution for Native<'_, S, T, W>
where
    S: crate::session_loop::Session + crate::tool_call::InputContext,
    T: Application,
    W: codetether_worker_tools::Host<Registry = ToolRegistry<T>>,
{
    type Tools = ToolRegistry<T>;
    fn tools(
        &self,
        provider: Arc<dyn Provider>,
        model: String,
        approval: AutoApprove,
        workspace: &Path,
        output: policy::OutputCallback,
    ) -> Self::Tools {
        codetether_worker_tools::create_filtered_registry::<W>(
            provider, model, approval, workspace, output,
        )
    }
    fn definitions(&self, tools: &Self::Tools) -> Vec<ToolDefinition> {
        tools.definitions()
    }
    fn system_prompt(&self, workspace: &Path) -> String {
        codetether_agent_core::builtin::build_system_prompt(workspace)
    }
    async fn run(
        &mut self,
        request: policy::RunRequest<'_, Self::Tools>,
    ) -> anyhow::Result<SessionStepsOutcome> {
        crate::run_session_steps(
            request.provider,
            self.session,
            request.model,
            request.system_prompt,
            request.tools,
            request.definitions,
            request.auto_approve,
            request.workspace,
            request.output,
            request.max_steps,
        )
        .await
    }
}
