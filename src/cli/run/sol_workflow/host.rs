//! Native provider, worker and diagnostics binding for the Sol workflow.
use super::{diagnostics, planner, worker::Worker};
use crate::cli::RunArgs;
use anyhow::Result;
use std::path::{Path, PathBuf};
pub(super) struct NativeHost;
#[async_trait::async_trait(?Send)]
impl codetether_sol_workflow::Host for NativeHost {
    type Worker = Worker;
    async fn complete(system: String, user: String) -> Result<String> {
        planner::complete(system, user).await
    }
    async fn start(args: &RunArgs, workspace: PathBuf) -> Result<Worker> {
        Worker::start(args, workspace).await
    }
    async fn collect(
        workspace: &Path,
        worker: &Worker,
    ) -> Result<Option<codetether_validation::ValidationReport>> {
        diagnostics::collect(workspace, &worker.session).await
    }
}
#[async_trait::async_trait(?Send)]
impl codetether_sol_workflow::Worker for Worker {
    fn model(&self) -> String {
        self.session.metadata.model.clone().unwrap_or_default()
    }
    fn session_id(&self) -> String {
        self.session.id.clone()
    }
    async fn execute(&mut self, prompt: &str) -> Result<String> {
        Worker::execute(self, prompt).await
    }
}
