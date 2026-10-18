use super::Native;
use crate::{
    provider::{Provider, ToolDefinition},
    tool::ToolRegistry,
};
use codetether_a2a_worker_core::{
    execution_policy::AutoApprove,
    session_protocol::{
        SessionStepsOutcome,
        policy::{Execution, OutputCallback, RunRequest},
    },
};
use std::{path::Path, sync::Arc};
impl Execution for Native<'_> {
    type Tools = ToolRegistry;
    fn tools(
        &self,
        provider: Arc<dyn Provider>,
        model: String,
        policy: AutoApprove,
        workspace: &Path,
        output: OutputCallback,
    ) -> ToolRegistry {
        super::super::super::create_filtered_registry(provider, model, policy, workspace, output)
    }
    fn definitions(&self, tools: &ToolRegistry) -> Vec<ToolDefinition> {
        tools.definitions()
    }
    fn system_prompt(&self, workspace: &Path) -> String {
        crate::agent::builtin::build_system_prompt(workspace)
    }
    async fn run(
        &mut self,
        request: RunRequest<'_, ToolRegistry>,
    ) -> anyhow::Result<SessionStepsOutcome> {
        super::super::super::run_session_steps(
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
