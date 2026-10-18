//! Native provider and loop bindings for the shared Ralph CLI.
use crate::{cli, provider, ralph};
struct NativeHost;
#[async_trait::async_trait(?Send)]
impl codetether_ralph_cli::Host for NativeHost {
    async fn load_provider(
        model: Option<String>,
    ) -> anyhow::Result<(std::sync::Arc<dyn provider::Provider>, String)> {
        let registry = provider::ProviderRegistry::from_vault().await?;
        codetether_ralph_cli::select_provider(model, &|name| registry.get(name))
    }
    async fn execute_loop(
        prd: std::path::PathBuf,
        provider: std::sync::Arc<dyn provider::Provider>,
        model: String,
        config: ralph::RalphConfig,
        json: bool,
    ) -> anyhow::Result<()> {
        let mut loop_runner = ralph::RalphLoop::new(prd, provider, model, config).await?;
        let result = loop_runner.run().await?;
        codetether_ralph_cli::print_result(&result, json, || loop_runner.status_markdown())
    }
}
pub(super) async fn execute(args: cli::RalphArgs) -> anyhow::Result<()> {
    codetether_ralph_cli::execute::<NativeHost>(args).await
}
