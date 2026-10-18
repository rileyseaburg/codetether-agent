//! Native dependency and frontend observation contracts.
use super::{LoopInput, SelectedProvider, WorkerContext};
use crate::{task_input::TaskInput, task_result::TaskResult};
use anyhow::Result;
use codetether_swarm_core::{SwarmEvent, agent_retry::AgentLoopExit, result_store::ResultStore};
use codetether_tool_core::ToolResult;
use std::{future::Future, sync::Arc};
use tokio::sync::mpsc;
/// Native observation of a direct swarm invocation.
/// # Implementors
/// Frontends supply lifecycle events while retaining their own cancellation cleanup.
pub trait Observer: Send {
    /// Start observing the ordered tasks before loading providers.
    fn begin(tasks: &[TaskInput], max_steps: usize) -> Self;
    /// Clone the event sender supplied to each worker.
    fn sender(&self) -> mpsc::Sender<SwarmEvent>;
    /// Publish the final ordered worker outcomes.
    fn complete(&mut self, results: &[TaskResult]);
}
/// Native dependencies required by shared direct-swarm execution.
/// # Implementors
/// Applications bind provider loading, restricted registries, session prompt
/// instructions and the provider/tool loop. Registry instances must be shareable.
pub trait Host: 'static {
    /// Concrete executable registry for a worker.
    type Registry: Send + Sync + 'static;
    /// Frontend lifecycle observer.
    type Observer: Observer;
    /// Load the requested model and report provider errors through the observer.
    fn load(
        requested: Option<&str>,
        observer: &mut Self::Observer,
    ) -> impl Future<Output = Result<Result<SelectedProvider, ToolResult>>> + Send;
    /// Construct the restricted worker registry with its shared result store.
    fn registry(context: &WorkerContext, store: Arc<ResultStore>) -> Arc<Self::Registry>;
    /// Return the provider-facing definitions for that same registry.
    fn definitions(registry: &Self::Registry) -> Vec<codetether_provider_core::ToolDefinition>;
    /// Compose repository and session-aware worker instructions.
    fn system_prompt(context: &WorkerContext, task: &TaskInput) -> String;
    /// Execute one worker with the prepared tools, prompts and event sender.
    fn run(
        input: LoopInput<Self::Registry>,
    ) -> impl Future<Output = Result<(String, usize, usize, AgentLoopExit)>> + Send;
}
