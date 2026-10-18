//! Application runtime operations for shared dependency-stage traversal.
use super::state::Run;
use anyhow::Result;
use async_trait::async_trait;
use codetether_swarm_core::stage_runtime::{StageInput, StageRuntime};
use codetether_swarm_core::{SubTask, SubTaskResult};
use std::time::Instant;
#[async_trait]
impl<H: crate::Host> StageRuntime for Run<'_, H> {
    fn subtasks(&self) -> &[SubTask] {
        &self.subtasks
    }
    async fn may_continue(&mut self) -> bool {
        super::control::may_continue(self).await
    }
    fn stage_input(&self, stage: usize) -> StageInput<'_> {
        super::stage_input::read(self, stage)
    }
    fn report_blocked(&self, tasks: &[SubTask], results: &[SubTaskResult]) {
        codetether_swarm_core::run_policy::report_blocked(tasks, results, |event| {
            self.executor.try_send_event(event);
        });
    }
    async fn execute_tasks(&mut self, tasks: Vec<SubTask>) -> Result<Vec<SubTaskResult>> {
        super::stage_operations::execute(self, tasks).await
    }
    async fn record_results(
        &mut self,
        stage: usize,
        started: Instant,
        results: Vec<SubTaskResult>,
    ) {
        super::stage_operations::record(self, stage, started, results).await;
    }
}
