//! Native policy and clone effects with the application forage hook.
use super::{Forage, WorkerTaskRuntime};
use anyhow::Result;
use async_trait::async_trait;
use codetether_a2a_worker_core::task_driver::special::SpecialTaskHost;
use serde_json::{Map, Value};
use std::marker::PhantomData;
pub(super) struct Native<'a, F> {
    pub runtime: &'a WorkerTaskRuntime,
    pub forage: PhantomData<fn() -> F>,
}
#[async_trait]
impl<F: Forage> SpecialTaskHost for Native<'_, F> {
    async fn evaluate(&self, tool: &str, args: &Value) -> Option<codetether_tool_core::ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(tool, args).await
    }
    async fn clone_repo(&self, task: &Value, metadata: &Map<String, Value>) -> Result<String> {
        codetether_a2a_worker_clone::handle_clone_repo_task(
            &self.runtime.client,
            &self.runtime.server,
            &self.runtime.token,
            &self.runtime.worker_id,
            task,
            metadata,
        )
        .await
    }
    async fn forage(
        &self,
        title: &str,
        prompt: &str,
        metadata: &Map<String, Value>,
        selected_model: Option<String>,
    ) -> Result<String> {
        F::execute(title, prompt, metadata, selected_model).await
    }
}
