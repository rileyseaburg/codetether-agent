//! Construct the original Ralph configuration after provider initialization.
use codetether_cli_options::RalphArgs;
pub(super) async fn execute<H: crate::Host>(args: RalphArgs) -> anyhow::Result<()> {
    tracing::info!("Starting Ralph loop with PRD: {}", args.prd.display());
    let (provider, model) = H::load_provider(args.model).await?;
    let config = codetether_ralph_core::RalphConfig {
        prd_path: args.prd.to_string_lossy().to_string(),
        max_iterations: args.max_iterations,
        ..Default::default()
    };

    H::execute_loop(args.prd.clone(), provider, model, config, args.json).await
}
