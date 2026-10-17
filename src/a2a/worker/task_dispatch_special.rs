//! Application host for shared special-task dispatch.
use super::{TaskContext, WorkerTaskRuntime, handle_clone_repo_task, handle_forage_task};
use anyhow::Result;
use async_trait::async_trait;
use codetether_a2a_worker_core::task_driver::special::{self, SpecialTaskHost};
use serde_json::{Map, Value};
struct Host<'a>(&'a WorkerTaskRuntime);
#[async_trait]
impl SpecialTaskHost for Host<'_> {
    async fn evaluate(&self, tool: &str, args: &Value) -> Option<crate::tool::ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(tool, args).await
    }
    async fn clone_repo(&self, task: &Value, metadata: &Map<String, Value>) -> Result<String> {
        handle_clone_repo_task(
            &self.0.client,
            &self.0.server,
            &self.0.token,
            &self.0.worker_id,
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
        handle_forage_task(title, prompt, metadata, selected_model).await
    }
}
pub(super) async fn dispatch_special_task(
    runtime: &WorkerTaskRuntime,
    task: &Value,
    title: &str,
    context: &TaskContext,
) -> Option<codetether_a2a_worker_core::outcome::TaskInner> {
    special::dispatch_special_task(&Host(runtime), task, title, context).await
}
