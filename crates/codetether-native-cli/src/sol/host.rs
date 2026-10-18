//! Native provider, worker and diagnostics binding for the Sol workflow.
use super::{Session, Worker};
use anyhow::Result;
use codetether_cli_options::RunArgs;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
/// Native Sol workflow using session S and validation services V.
/// # Examples
/// ```
/// let _: Option<codetether_native_cli::sol::Host<(), ()>> = None;
/// ```
pub struct Host<S, V>(PhantomData<fn() -> (S, V)>);
#[async_trait::async_trait]
impl<S: Session, V: codetether_validation::Host + Default + Send> codetether_sol_workflow::Host
    for Host<S, V>
{
    type Worker = Worker<S>;
    async fn complete(system: String, user: String) -> Result<String> {
        codetether_sol_workflow::planner::complete::<
            codetether_native_providers::registry::application::Native,
        >(system, user)
        .await
    }
    async fn start(args: &RunArgs, workspace: PathBuf) -> Result<Worker<S>> {
        Worker::start(args, workspace).await
    }
    async fn collect(
        workspace: &Path,
        worker: &Worker<S>,
    ) -> Result<Option<codetether_validation::ValidationReport>> {
        codetether_sol_workflow::diagnostics::collect(
            workspace,
            worker.session.tool_uses(),
            &V::default(),
        )
        .await
    }
}
#[async_trait::async_trait]
impl<S: Session> codetether_sol_workflow::Worker for Worker<S> {
    fn model(&self) -> String {
        self.session.model()
    }
    fn session_id(&self) -> String {
        codetether_run_resume::SessionSource::id(&self.session).to_owned()
    }
    async fn execute(&mut self, prompt: &str) -> Result<String> {
        Worker::execute(self, prompt).await
    }
}
