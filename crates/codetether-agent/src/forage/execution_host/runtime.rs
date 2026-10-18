//! Delegate prepared engine requests to existing application entry points.
use crate::swarm::SwarmExecutor as NativeSwarm;
use crate::{
    cli::RunArgs,
    okr::{Okr, OkrRun},
    provider::Provider,
    swarm::SwarmConfig,
};
use codetether_go_ralph::GoRalphResult;
use std::sync::Arc;
pub(crate) struct Application;
impl codetether_native_forage::Execution for Application {
    type Swarm = NativeSwarm;
    async fn run(args: RunArgs) -> anyhow::Result<()> {
        crate::cli::run::execute(args).await
    }
    fn swarm(config: SwarmConfig) -> NativeSwarm {
        NativeSwarm::new(config)
    }
    async fn go(
        task: &str,
        okr: &mut Okr,
        run: &mut OkrRun,
        provider: Arc<dyn Provider>,
        model: &str,
        max_iterations: usize,
        max_concurrent: usize,
    ) -> anyhow::Result<GoRalphResult> {
        crate::cli::go_ralph::execute_go_ralph(
            task,
            okr,
            run,
            provider,
            model,
            max_iterations,
            None,
            max_concurrent,
            None,
        )
        .await
    }
}
