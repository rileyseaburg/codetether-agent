//! Ralph CLI execution using the shared runtime loop and report presentation.
use crate::{Host, RalphLoop};
use codetether_provider_core::Provider;
use codetether_ralph_core::RalphConfig;
use std::{path::PathBuf, sync::Arc};

#[async_trait::async_trait(?Send)]
impl<H: Host> codetether_ralph_cli::LoopRunner for RalphLoop<H> {
    async fn execute(
        prd: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
        json: bool,
    ) -> anyhow::Result<()> {
        let mut loop_runner = RalphLoop::<H>::new(prd, provider, model, config).await?;
        let result = loop_runner.run().await?;
        codetether_ralph_cli::print_result(&result, json, || loop_runner.status_markdown())
    }
}
